#!/usr/bin/env node
import {
  createHash,
  createPrivateKey,
  createPublicKey,
  generateKeyPairSync,
  sign,
  verify,
} from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, posix, win32 } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const PKCS8_PREFIX = Buffer.from("302e020100300506032b657004220420", "hex");
const SPKI_PREFIX = Buffer.from("302a300506032b6570032100", "hex");
const MAGIC = Buffer.from("TSP1");

export function packPluginFiles(pluginJson, uiJson, wasm, icon, readme) {
  const out = [MAGIC];
  for (const c of [pluginJson, uiJson, wasm, icon, readme]) {
    const buf = Buffer.isBuffer(c) ? c : Buffer.from(c);
    const len = Buffer.alloc(4);
    len.writeUInt32BE(buf.length);
    out.push(len, buf);
  }
  return Buffer.concat(out);
}

export function authorIdFromPluginId(id) {
  const i = id.indexOf(".");
  return i === -1 ? id : id.slice(0, i);
}

function keyFromSeed(seed) {
  return createPrivateKey({
    key: Buffer.concat([PKCS8_PREFIX, seed]),
    format: "der",
    type: "pkcs8",
  });
}

export function seedFromKey(key) {
  const der = key.export({ format: "der", type: "pkcs8" });
  return der.subarray(der.length - 32);
}

export function generateSeed() {
  const { privateKey } = generateKeyPairSync("ed25519");
  return seedFromKey(privateKey);
}

export function publicKeyHexFromSeed(seed) {
  const spki = createPublicKey(keyFromSeed(seed)).export({
    format: "der",
    type: "spki",
  });
  return spki.subarray(-32).toString("hex");
}

export function signRelease({ seed, pluginId, abi, kind, authorId, userId, artifact }) {
  if (!Buffer.isBuffer(seed) || seed.length !== 32) {
    throw new Error("need a 32-byte author seed");
  }
  if (!/^[0-9a-f]{32}$/.test(userId || "")) {
    throw new Error("user_id must be 32 lowercase hex chars");
  }
  const payload_json = JSON.stringify({
    manifest: {
      plugin_id: pluginId,
      abi_version: abi,
      kind,
      author_id: authorId,
    },
    artifact_sha256_hex: createHash("sha256").update(artifact).digest("hex"),
    user_id: userId,
  });
  const signature_hex = sign(
    null,
    Buffer.from(payload_json, "utf8"),
    keyFromSeed(seed),
  ).toString("hex");
  return { payload_json, signature_hex };
}

export function verifyRelease(envelope, publicKeyHex, artifact) {
  const pk = Buffer.from(publicKeyHex, "hex");
  if (pk.length !== 32) return false;
  const key = createPublicKey({
    key: Buffer.concat([SPKI_PREFIX, pk]),
    format: "der",
    type: "spki",
  });
  if (
    !verify(
      null,
      Buffer.from(envelope.payload_json, "utf8"),
      key,
      Buffer.from(envelope.signature_hex, "hex"),
    )
  ) {
    return false;
  }
  const payload = JSON.parse(envelope.payload_json);
  if (payload.user_id && !/^[0-9a-f]{32}$/.test(payload.user_id)) return false;
  return (
    payload.artifact_sha256_hex ===
    createHash("sha256").update(artifact).digest("hex")
  );
}

export function identitySkPath({
  platform = process.platform,
  env = process.env,
  home = homedir(),
} = {}) {
  const j = platform === "win32" ? win32.join : posix.join;
  const tail = ["com.tianshu.desktop", "identity", "user.sk.hex"];
  if (platform === "darwin") {
    return j(home, "Library", "Application Support", ...tail);
  }
  if (platform === "win32") {
    const appdata = env.APPDATA || j(home, "AppData", "Roaming");
    return j(appdata, ...tail);
  }
  const data = env.XDG_DATA_HOME || j(home, ".local", "share");
  return j(data, ...tail);
}

export function userSkPathFromArgs(argv, opts) {
  const i = argv.findIndex((a) => a === "--sk" || a.startsWith("--sk="));
  if (i !== -1) {
    const a = argv[i];
    const path = a.startsWith("--sk=") ? a.slice(5) : argv[i + 1];
    if (!path || path.startsWith("-")) throw new Error("need a path after --sk");
    return path;
  }
  return identitySkPath(opts);
}

export function userIdFromArgs(argv) {
  const i = argv.findIndex((a) => a === "--user-id" || a.startsWith("--user-id="));
  if (i === -1) throw new Error("need --user-id (Tianshu account id)");
  const a = argv[i];
  const id = a.startsWith("--user-id=") ? a.slice(10) : argv[i + 1];
  if (!id || id.startsWith("-")) throw new Error("need an id after --user-id");
  if (!/^[0-9a-f]{32}$/.test(id)) throw new Error("user_id must be 32 lowercase hex chars");
  return id;
}

function loadSeed(path) {
  if (!existsSync(path)) {
    throw new Error("no Tianshu user key on this machine; sign in to Tianshu first");
  }
  const seed = Buffer.from(readFileSync(path, "utf8").trim(), "hex");
  if (seed.length !== 32) throw new Error("user key file is not 32 bytes hex");
  return seed;
}

function main() {
  const root = join(dirname(fileURLToPath(import.meta.url)), "..");
  const dist = join(root, "dist");
  const descPath = existsSync(join(dist, "plugin.json"))
    ? join(dist, "plugin.json")
    : join(root, "plugin.json");
  const desc = JSON.parse(readFileSync(descPath, "utf8"));
  const wasmPath = join(dist, "plugin.wasm");
  if (!existsSync(wasmPath)) {
    throw new Error("dist/plugin.wasm missing; build first");
  }
  const read = (name) => {
    const p = join(dist, name);
    return existsSync(p) ? readFileSync(p) : Buffer.alloc(0);
  };
  const artifact = packPluginFiles(
    read("plugin.json"),
    read("ui.json"),
    readFileSync(wasmPath),
    desc.icon ? read(desc.icon) : Buffer.alloc(0),
    desc.readme ? read(desc.readme) : Buffer.alloc(0),
  );
  const envelope = signRelease({
    seed: loadSeed(userSkPathFromArgs(process.argv)),
    pluginId: desc.id,
    abi: desc.abi,
    kind: "wasm",
    authorId: authorIdFromPluginId(desc.id),
    userId: userIdFromArgs(process.argv),
    artifact,
  });
  mkdirSync(dist, { recursive: true });
  const packName = `${desc.id}-${desc.version}.pack`;
  writeFileSync(join(dist, packName), artifact);
  writeFileSync(join(dist, "release.json"), `${JSON.stringify(envelope)}\n`);
  console.log("wrote", join("dist", packName));
  console.log("wrote dist/release.json");
}

const entry = process.argv[1] && pathToFileURL(process.argv[1]).href;
if (entry === import.meta.url) {
  try {
    main();
  } catch (e) {
    console.error(e instanceof Error ? e.message : e);
    process.exit(1);
  }
}
