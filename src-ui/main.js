const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const $ = (id) => document.getElementById(id);

const EMOJIS = ["📱", "📲", "📶", "💳", "⭐", "🔥", "🆓", "🎁"];
const LEVEL_TEXT = {
  normal: "状态正常",
  soon: "临期提醒",
  due: "今天到期",
  overdue: "已超期",
};

let app = null; // { view_mode, cards[] }，cards 已按剩余天数升序
let heroId = null; // 轮播当前停留在的卡 id（跨刷新保持）
let editingId = null; // 正在编辑的卡 id；null = 添加模式
let selectedEmoji = EMOJIS[0];
let deleteArmed = false;
let deleteTimer = null;

function escapeHtml(s) {
  return s.replace(/[&<>"']/g, (ch) =>
    ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[ch]
  );
}

function todayStr() {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

function daysText(c) {
  if (c.level === "due") return "今天";
  if (c.level === "overdue") return `超期 ${-c.days_left} 天`;
  return `${c.days_left} 天`;
}

// ---------- 渲染 ----------
function render() {
  if (!app) return;
  const hasCards = app.cards.length > 0;
  $("empty-view").classList.toggle("hidden", hasCards);
  $("app-view").classList.toggle("hidden", !hasCards);
  if (!hasCards) return;

  $("card-count").textContent = `${app.cards.length} 张卡`;

  const isList = app.view_mode !== "carousel";
  $("list-view").classList.toggle("hidden", !isList);
  $("carousel-view").classList.toggle("hidden", isList);
  $("view-list-btn").classList.toggle("active", isList);
  $("view-carousel-btn").classList.toggle("active", !isList);

  renderList();
  renderHero();
}

function renderList() {
  $("card-list").innerHTML = app.cards
    .map(
      (c) => `
      <button class="card-row ${c.level}" data-id="${c.id}">
        <span class="row-icon">${c.icon}</span>
        <span class="row-main">
          <b>${escapeHtml(c.name)}</b>
          <i>下次重置 ${c.next_reset}</i>
        </span>
        <span class="row-side">
          <b class="row-days">${daysText(c)}</b>
          <i>${LEVEL_TEXT[c.level] || c.level}</i>
        </span>
      </button>`
    )
    .join("");
}

function renderHero() {
  const cards = app.cards;
  let idx = heroId ? cards.findIndex((c) => c.id === heroId) : -1;
  if (idx < 0) idx = 0;
  heroId = cards[idx].id;
  const c = cards[idx];

  const card = $("hero-card");
  card.className = "card " + c.level;
  $("hero-icon").textContent = c.icon;
  $("hero-name").textContent = c.name;
  $("hero-chip").textContent = LEVEL_TEXT[c.level] || c.level;

  const days = $("hero-days");
  const unit = $("hero-unit");
  days.classList.remove("text-big");
  unit.classList.remove("hidden");

  if (c.level === "due") {
    days.textContent = "今天";
    days.classList.add("text-big");
    unit.classList.add("hidden");
    $("hero-label").textContent = "重置日就是今天，别忘了";
  } else if (c.level === "overdue") {
    days.textContent = -c.days_left;
    $("hero-label").textContent = "已超期，请立即重置";
  } else if (c.level === "soon") {
    days.textContent = c.days_left;
    $("hero-label").textContent = "距下次重置只剩";
  } else {
    days.textContent = c.days_left;
    $("hero-label").textContent = "距下次重置还有";
  }

  $("hero-next-date").textContent = c.next_reset;
  $("hero-last-date").textContent = c.last_reset;
  $("hero-interval-text").textContent = `${c.interval_days} 天`;

  const single = cards.length <= 1;
  $("hero-prev").classList.toggle("hidden", single);
  $("hero-next").classList.toggle("hidden", single);

  $("hero-dots").innerHTML = cards
    .map((_, i) => `<span class="dot${i === idx ? " on" : ""}" data-idx="${i}"></span>`)
    .join("");
  $("hero-dots").classList.toggle("hidden", single);
}

// ---------- 视图切换 ----------
function setViewMode(mode) {
  if (!app || app.view_mode === mode) return;
  app.view_mode = mode;
  render();
  invoke("set_view_mode", { mode }).catch(() => {});
}

function heroStep(delta) {
  const cards = app.cards;
  if (cards.length === 0) return;
  let idx = cards.findIndex((c) => c.id === heroId);
  if (idx < 0) idx = 0;
  idx = (idx + delta + cards.length) % cards.length;
  heroId = cards[idx].id;
  renderHero();
}

// ---------- 添加 / 编辑表单 ----------
function renderEmojiPicker() {
  $("f-icons").innerHTML = EMOJIS.map(
    (e) => `<button type="button" class="emoji${e === selectedEmoji ? " on" : ""}" data-emoji="${e}">${e}</button>`
  ).join("");
}

function openForm(card) {
  editingId = card ? card.id : null;
  selectedEmoji = card ? card.icon : EMOJIS[0];
  deleteArmed = false;
  clearTimeout(deleteTimer);

  $("form-title").textContent = card ? "编辑手机卡" : "添加手机卡";
  $("save-card-btn").textContent = card ? "保存修改" : "添加";
  $("f-name").value = card ? card.name : "";
  $("f-date").value = card ? card.last_reset : (app && app.cards.length ? app.cards[0].today : todayStr());
  $("f-interval").value = card ? card.interval_days : 180;
  $("f-remind").value = card ? card.remind_before_days : 3;
  $("delete-card-btn").classList.toggle("hidden", !card);
  $("delete-card-btn").textContent = "🗑 删除这张卡";
  $("form-error").classList.add("hidden");
  $("form-msg").classList.add("hidden");
  renderEmojiPicker();
  $("card-form").classList.remove("hidden");
  $("f-name").focus();
}

function closeForm() {
  $("card-form").classList.add("hidden");
  editingId = null;
  deleteArmed = false;
  clearTimeout(deleteTimer);
}

// ---------- 工具 ----------
function toast(text) {
  const el = $("toast");
  el.textContent = text;
  el.classList.remove("hidden");
  clearTimeout(toast._t);
  toast._t = setTimeout(() => el.classList.add("hidden"), 2600);
}

// ---------- 事件 ----------
$("empty-add").addEventListener("click", () => openForm(null));
$("add-card-btn").addEventListener("click", () => openForm(null));

$("view-list-btn").addEventListener("click", () => setViewMode("list"));
$("view-carousel-btn").addEventListener("click", () => setViewMode("carousel"));

$("card-list").addEventListener("click", (e) => {
  const row = e.target.closest(".card-row");
  if (!row) return;
  heroId = row.dataset.id;
  setViewMode("carousel");
});

$("hero-prev").addEventListener("click", () => heroStep(-1));
$("hero-next").addEventListener("click", () => heroStep(1));

$("hero-dots").addEventListener("click", (e) => {
  const dot = e.target.closest(".dot");
  if (!dot) return;
  const idx = parseInt(dot.dataset.idx, 10);
  if (Number.isFinite(idx) && app && app.cards[idx]) {
    heroId = app.cards[idx].id;
    renderHero();
  }
});

$("hero-reset").addEventListener("click", async () => {
  if (!heroId) return;
  try {
    app = await invoke("reset_card", { cardId: heroId });
    render();
    const c = app.cards.find((x) => x.id === heroId);
    if (c) toast(`已重置，下次重置：${c.next_reset}`);
  } catch (e) {
    toast(String(e));
  }
});

$("hero-edit").addEventListener("click", () => {
  if (!app || !heroId) return;
  const c = app.cards.find((x) => x.id === heroId);
  if (c) openForm(c);
});

$("close-form").addEventListener("click", closeForm);

$("f-icons").addEventListener("click", (e) => {
  const btn = e.target.closest(".emoji");
  if (!btn) return;
  selectedEmoji = btn.dataset.emoji;
  renderEmojiPicker();
});

$("save-card-btn").addEventListener("click", async () => {
  const err = $("form-error");
  const msg = $("form-msg");
  err.classList.add("hidden");
  msg.classList.add("hidden");

  const name = $("f-name").value.trim();
  const date = $("f-date").value;
  const interval = parseInt($("f-interval").value, 10);
  const remind = parseInt($("f-remind").value, 10);

  if (!name) {
    err.textContent = "请填写卡名称";
    err.classList.remove("hidden");
    return;
  }
  if (!date) {
    err.textContent = "请选择上次重置的日期";
    err.classList.remove("hidden");
    return;
  }
  if (!Number.isFinite(interval) || interval < 1 || interval > 3650) {
    err.textContent = "周期必须是 1 ~ 3650 的天数";
    err.classList.remove("hidden");
    return;
  }
  if (!Number.isFinite(remind) || remind < 0 || remind > 30) {
    err.textContent = "提前提醒天数必须是 0 ~ 30";
    err.classList.remove("hidden");
    return;
  }

  try {
    if (editingId) {
      app = await invoke("save_card", {
        cardId: editingId,
        name,
        icon: selectedEmoji,
        intervalDays: interval,
        remindBeforeDays: remind,
        lastReset: date,
      });
      toast("已保存");
    } else {
      app = await invoke("add_card", {
        name,
        icon: selectedEmoji,
        firstReset: date,
        intervalDays: interval,
      });
      toast("已添加，开始倒计时");
    }
    closeForm();
    render();
  } catch (e) {
    err.textContent = String(e);
    err.classList.remove("hidden");
  }
});

$("delete-card-btn").addEventListener("click", async () => {
  if (!editingId) return;
  if (!deleteArmed) {
    // 两步确认：第一次点击只是武装，3 秒内再点才真正删除
    deleteArmed = true;
    $("delete-card-btn").textContent = "再点一次，确认删除";
    clearTimeout(deleteTimer);
    deleteTimer = setTimeout(() => {
      deleteArmed = false;
      $("delete-card-btn").textContent = "🗑 删除这张卡";
    }, 3000);
    return;
  }
  try {
    app = await invoke("delete_card", { cardId: editingId });
    closeForm();
    render();
    toast("已删除这张卡");
  } catch (e) {
    $("form-error").textContent = String(e);
    $("form-error").classList.remove("hidden");
  }
});

// 设置（全局）：仅开机自启
$("open-settings").addEventListener("click", () => {
  $("settings-msg").classList.add("hidden");
  invoke("get_autostart")
    .then((on) => ($("set-autostart").checked = on))
    .catch(() => ($("set-autostart").checked = false));
  $("settings").classList.remove("hidden");
});
$("close-settings").addEventListener("click", () => $("settings").classList.add("hidden"));

$("save-settings").addEventListener("click", async () => {
  const msg = $("settings-msg");
  try {
    await invoke("set_autostart", { enabled: $("set-autostart").checked });
    msg.textContent = "✓ 已保存";
    msg.style.color = "";
  } catch (e) {
    msg.textContent = String(e);
    msg.style.color = "var(--red)";
  }
  msg.classList.remove("hidden");
  setTimeout(() => msg.classList.add("hidden"), 2500);
});

// ---------- 全局事件与初始化 ----------
listen("cards-changed", (e) => {
  app = e.payload;
  render();
}).catch(() => {});

invoke("get_cards")
  .then((st) => {
    app = st;
    render();
  })
  .catch((e) => toast(String(e)));
