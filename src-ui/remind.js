const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const $ = (id) => document.getElementById(id);

function render(st) {
  if (!st.configured) return;
  const icon = $("r-icon");
  if (st.level === "due") {
    icon.textContent = "⏰";
    $("r-title").textContent = "手机卡今天到期";
    $("r-sub").textContent = "今天就是重置日，记得重置哦";
  } else if (st.level === "overdue") {
    icon.textContent = "🚨";
    $("r-title").textContent = `手机卡已超期 ${-st.days_left} 天`;
    $("r-sub").textContent = "再不重置可能要停机啦";
  } else {
    icon.textContent = "🔔";
    $("r-title").textContent = `手机卡还有 ${st.days_left} 天到期`;
    $("r-sub").textContent = `下次重置日期：${st.next_reset}`;
  }
}

$("r-reset").addEventListener("click", () => {
  invoke("reset_today").catch(() => {});
});

$("r-dismiss").addEventListener("click", () => {
  invoke("dismiss_reminder").catch(() => {});
});

listen("status-changed", (e) => {
  render(e.payload);
  if (e.payload.level === "normal") {
    // 重置成功后弹窗会被后端关闭，这里兜底
    invoke("dismiss_reminder").catch(() => {});
  }
}).catch(() => {});

invoke("get_status").then(render).catch(() => {});
