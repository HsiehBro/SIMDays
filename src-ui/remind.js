const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { getCurrentWindow, LogicalSize } = window.__TAURI__.window;

const win = getCurrentWindow();
const $ = (id) => document.getElementById(id);

function escapeHtml(s) {
  return s.replace(/[&<>"']/g, (ch) =>
    ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[ch]
  );
}

function statusText(c) {
  if (c.level === "due") return { text: "今天到期，记得重置", cls: "due" };
  if (c.level === "overdue") return { text: `已超期 ${-c.days_left} 天`, cls: "overdue" };
  return { text: `${c.days_left} 天后到期`, cls: "soon" };
}

function render(cards) {
  const due = cards.filter((c) => c.level !== "normal");
  if (due.length === 0) {
    // 全部处理完：直接隐藏提醒窗
    win.hide().catch(() => {});
    return;
  }

  $("r-icon").textContent = due.some((c) => c.level === "overdue") ? "🚨" : "🔔";
  $("r-title").textContent =
    due.length === 1 ? "这张手机卡需要关注" : `有 ${due.length} 张手机卡需要关注`;

  $("r-list").innerHTML = due
    .map((c) => {
      const s = statusText(c);
      return `
      <div class="popup-row ${s.cls}" data-id="${c.id}">
        <span class="pr-icon">${c.icon}</span>
        <span class="pr-main">
          <b>${escapeHtml(c.name)}</b>
          <i>${s.text} · ${c.next_reset} 重置</i>
        </span>
        <button class="btn primary pr-reset" data-id="${c.id}">✓ 已重置</button>
      </div>`;
    })
    .join("");

  // 按行数自适应窗口高度（列表超出上限后内部滚动）
  const height = Math.min(150 + due.length * 58 + 14, 400);
  win.setSize(new LogicalSize(340, height)).catch(() => {});
}

$("r-list").addEventListener("click", async (e) => {
  const btn = e.target.closest(".pr-reset");
  if (!btn) return;
  btn.disabled = true;
  try {
    const st = await invoke("reset_card", { cardId: btn.dataset.id });
    render(st.cards);
  } catch (err) {
    btn.disabled = false;
  }
});

$("r-dismiss").addEventListener("click", () => {
  invoke("dismiss_reminder").catch(() => {});
  win.hide().catch(() => {});
});

// 主窗口里重置/编辑卡时，这里同步刷新；列表清空则自动隐藏
listen("cards-changed", (e) => render(e.payload.cards)).catch(() => {});

invoke("get_due_cards").then(render).catch(() => {});
