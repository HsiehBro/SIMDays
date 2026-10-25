const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const $ = (id) => document.getElementById(id);

const LEVEL_TEXT = {
  normal: "状态正常",
  soon: "临期提醒",
  due: "今天到期",
  overdue: "已超期",
};

let lastStatus = null;

function render(st) {
  lastStatus = st;
  const setupView = $("setup-view");
  const mainView = $("main-view");

  if (!st.configured) {
    setupView.classList.remove("hidden");
    mainView.classList.add("hidden");
    if (!$("setup-date").value) $("setup-date").value = st.today;
    return;
  }
  setupView.classList.add("hidden");
  mainView.classList.remove("hidden");

  $("status-card").className = "card " + st.level;
  $("status-chip").textContent = LEVEL_TEXT[st.level] || st.level;

  const days = $("days");
  const unit = $("days-unit");
  days.classList.remove("text-big");
  unit.classList.remove("hidden");

  if (st.level === "due") {
    days.textContent = "今天";
    days.classList.add("text-big");
    unit.classList.add("hidden");
    $("count-label").textContent = "重置日就是今天，别忘了";
  } else if (st.level === "overdue") {
    days.textContent = -st.days_left;
    $("count-label").textContent = "已超期，请立即重置";
  } else if (st.level === "soon") {
    days.textContent = st.days_left;
    $("count-label").textContent = "距下次重置只剩";
  } else {
    days.textContent = st.days_left;
    $("count-label").textContent = "距下次重置还有";
  }

  $("next-date").textContent = st.next_reset;
  $("last-date").textContent = st.last_reset;
  $("interval-text").textContent = st.interval_days + " 天";
}

function toast(text) {
  const el = $("toast");
  el.textContent = text;
  el.classList.remove("hidden");
  clearTimeout(toast._t);
  toast._t = setTimeout(() => el.classList.add("hidden"), 2600);
}

function openSettings() {
  if (!lastStatus) return;
  $("set-last-reset").value = lastStatus.last_reset;
  $("set-interval").value = lastStatus.interval_days;
  $("set-remind").value = lastStatus.remind_before_days;
  $("settings-msg").classList.add("hidden");
  invoke("get_autostart")
    .then((on) => ($("set-autostart").checked = on))
    .catch(() => ($("set-autostart").checked = false));
  $("settings").classList.remove("hidden");
}

// ---------- 事件 ----------
$("setup-btn").addEventListener("click", async () => {
  const date = $("setup-date").value;
  const interval = parseInt($("setup-interval").value, 10);
  const err = $("setup-error");
  err.classList.add("hidden");
  if (!date) {
    err.textContent = "请选择上次重置的日期";
    err.classList.remove("hidden");
    return;
  }
  if (!Number.isFinite(interval) || interval < 1) {
    err.textContent = "周期必须是大于 0 的天数";
    err.classList.remove("hidden");
    return;
  }
  try {
    const st = await invoke("setup", { firstReset: date, intervalDays: interval });
    render(st);
    toast("设置完成，开始倒计时");
  } catch (e) {
    err.textContent = String(e);
    err.classList.remove("hidden");
  }
});

$("reset-btn").addEventListener("click", async () => {
  try {
    const st = await invoke("reset_today");
    render(st);
    toast(`已重置，下次重置：${st.next_reset}`);
  } catch (e) {
    toast(String(e));
  }
});

$("toggle-settings").addEventListener("click", openSettings);
$("close-settings").addEventListener("click", () => $("settings").classList.add("hidden"));

let saveMsgTimer = null;

$("save-settings").addEventListener("click", async () => {
  const msg = $("settings-msg");
  msg.textContent = "✓ 已保存";
  msg.style.color = "";
  try {
    const st = await invoke("save_settings", {
      intervalDays: parseInt($("set-interval").value, 10),
      remindBeforeDays: parseInt($("set-remind").value, 10),
      lastReset: $("set-last-reset").value,
    });
    await invoke("set_autostart", { enabled: $("set-autostart").checked });
    render(st);
    msg.classList.remove("hidden");
    clearTimeout(saveMsgTimer);
    saveMsgTimer = setTimeout(() => msg.classList.add("hidden"), 2500);
  } catch (e) {
    msg.textContent = String(e);
    msg.style.color = "var(--red)";
    msg.classList.remove("hidden");
  }
});

listen("status-changed", (e) => render(e.payload)).catch(() => {});

// ---------- 初始化 ----------
invoke("get_status").then(render).catch((e) => toast(String(e)));
