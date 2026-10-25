#!/usr/bin/env node
// 生成应用图标与托盘三色图标（纯 Node 实现，无第三方依赖）
// 用法：node tools/gen_icons.js
const fs = require("fs");
const path = require("path");
const zlib = require("zlib");

// ---------- PNG 编码 ----------
const CRC_TABLE = (() => {
  const t = new Uint32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    t[n] = c >>> 0;
  }
  return t;
})();

function crc32(buf) {
  let c = 0xffffffff;
  for (const b of buf) c = CRC_TABLE[(c ^ b) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const body = Buffer.concat([Buffer.from(type, "ascii"), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(body));
  return Buffer.concat([len, body, crc]);
}

function writePng(file, w, h, rgba) {
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(w, 0);
  ihdr.writeUInt32BE(h, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // RGBA
  const raw = Buffer.alloc((w * 4 + 1) * h);
  for (let y = 0; y < h; y++) {
    raw[y * (w * 4 + 1)] = 0; // filter: none
    rgba.copy(raw, y * (w * 4 + 1) + 1, y * w * 4, (y + 1) * w * 4);
  }
  const png = Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk("IHDR", ihdr),
    chunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ]);
  fs.writeFileSync(file, png);
  console.log("written:", file, `${w}x${h}`);
}

// ---------- 2D SDF ----------
function sdRoundRect(px, py, cx, cy, hw, hh, r) {
  const qx = Math.abs(px - cx) - (hw - r);
  const qy = Math.abs(py - cy) - (hh - r);
  return (
    Math.min(Math.max(qx, qy), 0) +
    Math.hypot(Math.max(qx, 0), Math.max(qy, 0)) -
    r
  );
}

function sdSeg(px, py, ax, ay, bx, by) {
  const abx = bx - ax;
  const aby = by - ay;
  const t = Math.max(
    0,
    Math.min(1, ((px - ax) * abx + (py - ay) * aby) / (abx * abx + aby * aby))
  );
  return Math.hypot(px - (ax + abx * t), py - (ay + aby * t));
}

// ---------- 绘制：彩色底形 + 白色对勾 ----------
// shape: "rect"（圆角方块，应用图标）| "circle"（圆形，托盘图标）
const CHECK = [
  [0.32, 0.53],
  [0.45, 0.66],
  [0.7, 0.37],
];
const CHECK_HALF = 0.043; // 对勾线宽的一半（归一化）

function drawIcon(file, size, rgb, shape) {
  const W = size;
  const img = Buffer.alloc(W * W * 4);
  const segs = [];
  for (let i = 0; i + 1 < CHECK.length; i++) segs.push([CHECK[i], CHECK[i + 1]]);

  for (let y = 0; y < W; y++) {
    for (let x = 0; x < W; x++) {
      const nx = (x + 0.5) / W;
      const ny = (y + 0.5) / W;
      const dShape =
        shape === "circle"
          ? Math.hypot(nx - 0.5, ny - 0.5) - 0.5
          : sdRoundRect(nx, ny, 0.5, 0.5, 0.5, 0.5, 0.21);
      let dCheck = Infinity;
      for (const [a, b] of segs) {
        dCheck = Math.min(dCheck, sdSeg(nx, ny, a[0], a[1], b[0], b[1]));
      }
      // SDF 解析式抗锯齿：边缘羽化 0.5px
      const aBg = Math.max(0, Math.min(1, -dShape * W + 0.5));
      const aFg = Math.max(0, Math.min(1, (CHECK_HALF - dCheck) * W + 0.5));
      const outA = Math.min(1, aFg + aBg * (1 - aFg));
      const o = (y * W + x) * 4;
      if (outA <= 0) {
        img[o] = img[o + 1] = img[o + 2] = img[o + 3] = 0;
        continue;
      }
      img[o] = Math.round((255 * aFg + rgb[0] * aBg * (1 - aFg)) / outA);
      img[o + 1] = Math.round((255 * aFg + rgb[1] * aBg * (1 - aFg)) / outA);
      img[o + 2] = Math.round((255 * aFg + rgb[2] * aBg * (1 - aFg)) / outA);
      img[o + 3] = Math.round(outA * 255);
    }
  }
  writePng(file, W, W, img);
}

const GREEN = [22, 163, 74]; // #16A34A
const AMBER = [245, 158, 11]; // #F59E0B
const RED = [220, 38, 38]; // #DC2626

const root = path.join(__dirname, "..");
const iconsDir = path.join(root, "src-tauri", "icons");
fs.mkdirSync(iconsDir, { recursive: true });

drawIcon(path.join(__dirname, "app-icon.png"), 1024, GREEN, "rect");
drawIcon(path.join(iconsDir, "tray_green.png"), 128, GREEN, "circle");
drawIcon(path.join(iconsDir, "tray_yellow.png"), 128, AMBER, "circle");
drawIcon(path.join(iconsDir, "tray_red.png"), 128, RED, "circle");
console.log("done.");
