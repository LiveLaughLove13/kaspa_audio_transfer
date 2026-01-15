
      const errorEl = document.getElementById("error");
      const sendBtn = document.getElementById("sendBtn");
      const sendFileEl = document.getElementById("sendFile");
      const sendDropZoneEl = document.getElementById("sendDropZone");
      const sendFileHintEl = document.getElementById("sendFileHint");
      const sendPrivEl = document.getElementById("sendPriv");
      const sendPrivAutoHintEl = document.getElementById("sendPrivAutoHint");
      const sendToEl = document.getElementById("sendTo");
      const sendToResolvedEl = document.getElementById("sendToResolved");
      const sendAmountEl = document.getElementById("sendAmount");
      const sendRpcEl = document.getElementById("sendRpc");
      const sendProgressEl = document.getElementById("sendProgress");
      const sendTxidEl = document.getElementById("sendTxid");
      const sendRingEl = document.getElementById("sendRing");
      const sendExplorerLinkEl = document.getElementById("sendExplorerLink");

      const recvBtn = document.getElementById("recvBtn");
      const recvTxEl = document.getElementById("recvTx");
      const recvStartEl = document.getElementById("recvStart");
      const recvRpcEl = document.getElementById("recvRpc");
      const recvOutEl = document.getElementById("recvOut");
      const recvStatusEl = document.getElementById("recvStatus");
      const recvRingEl = document.getElementById("recvRing");
      const explorerTipEl = document.getElementById("explorerTip");
      const openBtn = document.getElementById("openBtn");
      const revealBtn = document.getElementById("revealBtn");

      const sendCopyTxidBtnEl = document.getElementById("sendCopyTxidBtn");
      const recvCopyTxBtnEl = document.getElementById("recvCopyTxBtn");
      const recvCopyStartBtnEl = document.getElementById("recvCopyStartBtn");

      const clearSessionBtnEl = document.getElementById("clearSessionBtn");

      const navWalletEl = document.getElementById("navWallet");
      const navTransferEl = document.getElementById("navTransfer");
      const navStudioEl = document.getElementById("navStudio");
      const pageWalletEl = document.getElementById("pageWallet");
      const pageTransferEl = document.getElementById("pageTransfer");
      const pageStudioEl = document.getElementById("pageStudio");
      const pageTitleEl = document.getElementById("pageTitle");
      const tabSendEl = document.getElementById("tabSend");
      const tabReceiveEl = document.getElementById("tabReceive");
      const transferSendPanelEl = document.getElementById("transferSendPanel");
      const transferReceivePanelEl = document.getElementById("transferReceivePanel");

      const studioTabAudioEl = document.getElementById("studioTabAudio");
      const studioTabVideoEl = document.getElementById("studioTabVideo");
      const studioAudioPanelEl = document.getElementById("studioAudioPanel");
      const studioVideoPanelEl = document.getElementById("studioVideoPanel");

      const studioAudioMicEl = document.getElementById("studioAudioMic");
      const studioAudioStartEl = document.getElementById("studioAudioStart");
      const studioAudioStopEl = document.getElementById("studioAudioStop");
      const studioAudioDeleteEl = document.getElementById("studioAudioDelete");
      const studioAudioSaveEl = document.getElementById("studioAudioSave");
      const studioAudioExportMp3El = document.getElementById("studioAudioExportMp3");
      const studioAudioExportWavEl = document.getElementById("studioAudioExportWav");
      const studioAudioSendEl = document.getElementById("studioAudioSend");
      const studioAudioStatusEl = document.getElementById("studioAudioStatus");
      const studioAudioPlaybackEl = document.getElementById("studioAudioPlayback");
      const studioAudioMetaEl = document.getElementById("studioAudioMeta");
      const studioAudioPrivEl = document.getElementById("studioAudioPriv");
      const studioAudioPrivAutoHintEl = document.getElementById("studioAudioPrivAutoHint");
      const studioAudioToEl = document.getElementById("studioAudioTo");
      const studioAudioToResolvedEl = document.getElementById("studioAudioToResolved");
      const studioAudioAmountEl = document.getElementById("studioAudioAmount");
      const studioAudioRpcEl = document.getElementById("studioAudioRpc");

      const studioVideoSourceEl = document.getElementById("studioVideoSource");
      const studioVideoCamEl = document.getElementById("studioVideoCam");
      const studioVideoMicEl = document.getElementById("studioVideoMic");
      const studioVideoStartEl = document.getElementById("studioVideoStart");
      const studioVideoStopEl = document.getElementById("studioVideoStop");
      const studioVideoDeleteEl = document.getElementById("studioVideoDelete");
      const studioVideoSaveEl = document.getElementById("studioVideoSave");
      const studioVideoExportMp4El = document.getElementById("studioVideoExportMp4");
      const studioVideoSendEl = document.getElementById("studioVideoSend");
      const studioVideoStatusEl = document.getElementById("studioVideoStatus");
      const studioVideoLiveEl = document.getElementById("studioVideoLive");
      const studioVideoPlaybackEl = document.getElementById("studioVideoPlayback");
      const studioVideoMetaEl = document.getElementById("studioVideoMeta");
      const studioVideoPrivEl = document.getElementById("studioVideoPriv");
      const studioVideoPrivAutoHintEl = document.getElementById("studioVideoPrivAutoHint");
      const studioVideoToEl = document.getElementById("studioVideoTo");
      const studioVideoToResolvedEl = document.getElementById("studioVideoToResolved");
      const studioVideoAmountEl = document.getElementById("studioVideoAmount");
      const studioVideoRpcEl = document.getElementById("studioVideoRpc");

      const walletStatusEl = document.getElementById("walletStatus");
      const walletUnlockedEl = document.getElementById("walletUnlocked");
      const walletProfileSelectEl = document.getElementById("walletProfileSelect");
      const walletRefreshBtnEl = document.getElementById("walletRefreshBtn");
      const walletPasswordEl = document.getElementById("walletPassword");
      const walletUnlockBtnEl = document.getElementById("walletUnlockBtn");
      const walletLockBtnEl = document.getElementById("walletLockBtn");

      const walletTabOverviewEl = document.getElementById("walletTabOverview");
      const walletTabManageEl = document.getElementById("walletTabManage");
      const walletTabAdvancedEl = document.getElementById("walletTabAdvanced");
      const walletOverviewPanelEl = document.getElementById("walletOverviewPanel");
      const walletManagePanelEl = document.getElementById("walletManagePanel");
      const walletAdvancedPanelEl = document.getElementById("walletAdvancedPanel");

      const walletShellEl = document.getElementById("walletShell");

      const walletHdrCreateEl = document.getElementById("walletHdrCreate");
      const walletHdrImportEl = document.getElementById("walletHdrImport");
      const walletHdrDeleteEl = document.getElementById("walletHdrDelete");

      const walletManageCreateSectionEl = document.getElementById("walletManageCreateSection");
      const walletManageImportSectionEl = document.getElementById("walletManageImportSection");
      const walletManageDataSectionEl = document.getElementById("walletManageDataSection");

      const walletNewUsernameEl = document.getElementById("walletNewUsername");
      const walletNewPasswordEl = document.getElementById("walletNewPassword");
      const walletWordCountEl = document.getElementById("walletWordCount");
      const walletCreateBtnEl = document.getElementById("walletCreateBtn");

      const walletImportUsernameEl = document.getElementById("walletImportUsername");
      const walletImportPasswordEl = document.getElementById("walletImportPassword");
      const walletImportMnemonicEl = document.getElementById("walletImportMnemonic");
      const walletMnemonicPassEl = document.getElementById("walletMnemonicPass");
      const walletImportMnemonicBtnEl = document.getElementById("walletImportMnemonicBtn");
      const walletImportPrivKeyEl = document.getElementById("walletImportPrivKey");
      const walletImportPrivKeyBtnEl = document.getElementById("walletImportPrivKeyBtn");

      const walletDeleteProfileBtnEl = document.getElementById("walletDeleteProfileBtn");
      const walletClearAllProfilesBtnEl = document.getElementById("walletClearAllProfilesBtn");

      const walletNetworkEl = document.getElementById("walletNetwork");
      const walletRpcUrlEl = document.getElementById("walletRpcUrl");
      const walletRpcModeLocalEl = document.getElementById("walletRpcModeLocal");
      const walletRpcModePublicEl = document.getElementById("walletRpcModePublic");
      const walletBasePathEl = document.getElementById("walletBasePath");
      const walletChainEl = document.getElementById("walletChain");
      const walletAddrIndexEl = document.getElementById("walletAddrIndex");
      const walletDeriveBtnEl = document.getElementById("walletDeriveBtn");
      const walletFullPathEl = document.getElementById("walletFullPath");
      const walletDerivedAddressEl = document.getElementById("walletDerivedAddress");

      const walletBalanceEl = document.getElementById("walletBalance");
      const walletBalanceRefreshBtnEl = document.getElementById("walletBalanceRefreshBtn");
      const walletSendToEl = document.getElementById("walletSendTo");
      const walletSendToResolvedEl = document.getElementById("walletSendToResolved");
      const walletSendAmountEl = document.getElementById("walletSendAmount");
      const walletSendKasBtnEl = document.getElementById("walletSendKasBtn");
      const walletSendKasTxidEl = document.getElementById("walletSendKasTxid");

      const walletCopyPathBtnEl = document.getElementById("walletCopyPathBtn");
      const walletCopyAddressBtnEl = document.getElementById("walletCopyAddressBtn");
      const walletCopySendKasTxidBtnEl = document.getElementById("walletCopySendKasTxidBtn");

      const walletDebugBtnEl = document.getElementById("walletDebugBtn");
      const walletDebugOutEl = document.getElementById("walletDebugOut");

      const nodeBundleDirOutEl = document.getElementById("nodeBundleDirOut");
      const nodeBundleImportBtnEl = document.getElementById("nodeBundleImportBtn");
      const nodeBundleOpenBtnEl = document.getElementById("nodeBundleOpenBtn");
      const nodeBundleClearBtnEl = document.getElementById("nodeBundleClearBtn");

      const modalOverlayEl = document.getElementById("modalOverlay");
      const modalCloseEl = document.getElementById("modalClose");
      const modalTitleEl = document.getElementById("modalTitle");
      const modalBodyEl = document.getElementById("modalBody");
      const modalActionsEl = document.getElementById("modalActions");

      const pillTextEl = document.getElementById("pillText");
      const rpcConnTextEl = document.getElementById("rpcConnText");

      const walletTxRefreshBtnEl = document.getElementById("walletTxRefreshBtn");
      const walletTxHistoryStatusEl = document.getElementById("walletTxHistoryStatus");
      const walletTxReceivedListEl = document.getElementById("walletTxReceivedList");
      const walletTxSentListEl = document.getElementById("walletTxSentList");

      const walletHeroBalanceEl = document.getElementById("walletHeroBalance");

      let lastReceivedPath = "";
      let selectedSendFile = null;
      let selectedSendFilePath = "";

      let nodeBundleDir = "";

      const RING_R = 18;
      const RING_CIRC = 2 * Math.PI * RING_R;

      function setRing(circleEl, pct) {
        const p = Number.isFinite(pct) ? Math.max(0, Math.min(1, pct)) : 0;
        if (!circleEl) return;
        circleEl.style.strokeDasharray = String(RING_CIRC);
        circleEl.style.strokeDashoffset = String(RING_CIRC * (1 - p));
      }

      function getExplorerTxUrl(txid) {
        const t = String(txid || "").trim();
        if (!t) return "https://explorer.kaspa.org/transactions/";
        return `https://explorer.kaspa.org/transactions/${t}`;
      }

      function formatKasFromSompi(sompi) {
        const v = Number(sompi || 0);
        if (!Number.isFinite(v)) return "0";
        const kas = v / 100000000;
        const fixed = kas.toFixed(8);
        return fixed.replace(/\.0+$/, "").replace(/(\.[0-9]*?)0+$/, "$1");
      }

      async function openExternal(url) {
        const opener = window.__TAURI__?.opener;
        if (opener?.openUrl) {
          await opener.openUrl(url);
          return;
        }
        window.open(url, "_blank");
      }

      let pillTimer = null;
      function flashPill(text) {
        if (!pillTextEl) return;
        const prev = pillTextEl.textContent;
        pillTextEl.textContent = text;
        if (pillTimer) clearTimeout(pillTimer);
        pillTimer = setTimeout(() => {
          pillTextEl.textContent = prev || "Ready";
          pillTimer = null;
        }, 1200);
      }

      async function copyText(text) {
        const value = String(text || "").trim();
        if (!value || value === "—") throw new Error("Nothing to copy");

        const tauriClipboard = window.__TAURI__?.clipboardManager;
        if (tauriClipboard?.writeText) {
          await tauriClipboard.writeText(value);
          return;
        }

        if (navigator.clipboard?.writeText) {
          await navigator.clipboard.writeText(value);
          return;
        }

        const el = document.createElement("textarea");
        el.value = value;
        el.setAttribute("readonly", "");
        el.style.position = "fixed";
        el.style.opacity = "0";
        el.style.left = "-9999px";
        document.body.appendChild(el);
        el.select();
        document.execCommand("copy");
        document.body.removeChild(el);
      }

      function showModal({ title, body, actions }) {
        if (!modalOverlayEl || !modalTitleEl || !modalBodyEl || !modalActionsEl) return;

        modalTitleEl.textContent = title || "Notification";
        modalBodyEl.textContent = body || "";
        modalActionsEl.innerHTML = "";

        (actions || []).forEach((a) => {
          const b = document.createElement("button");
          b.type = "button";
          b.className = `modalBtn ${a && a.primary ? "modalBtnPrimary" : ""}`.trim();
          b.textContent = (a && a.label) || "OK";
          b.addEventListener("click", async () => {
            try {
              if (a && a.onClick) await a.onClick();
            } finally {
              if (!(a && a.keepOpen)) closeModal();
            }
          });
          modalActionsEl.appendChild(b);
        });

        modalOverlayEl.style.display = "flex";
      }

      function closeModal() {
        modalOverlayEl.style.display = "none";
        modalTitleEl.textContent = "Notification";
        modalBodyEl.textContent = "";
        modalActionsEl.innerHTML = "";
      }

      function setError(msg) {
        if (!msg) {
          errorEl.style.display = "none";
          errorEl.textContent = "";
          return;
        }
        errorEl.style.display = "block";
        errorEl.textContent = msg;
      }

      window.addEventListener("error", (e) => {
        try {
          const err = e && e.error;
          const msg = err ? String(err.stack || err.message || err) : String((e && e.message) || e);
          setError(msg);
        } catch (_) {}
      });

      window.addEventListener("unhandledrejection", (e) => {
        try {
          setError(String((e && e.reason) || e));
        } catch (_) {}
      });

      async function readFileB64(file) {
        return new Promise((resolve, reject) => {
          const r = new FileReader();
          r.onerror = () => reject(new Error("failed reading file"));

          r.onload = () => resolve(String(r.result || ""));
          r.readAsDataURL(file);
        });
      }

      async function init() {
        const tauri = window.__TAURI__?.core || window.__TAURI__?.tauri;
        const eventApi = window.__TAURI__?.event;
        const dialogApi = window.__TAURI__?.dialog;
        const fsApi = window.__TAURI__?.fs;

        let isWalletSessionUnlocked = false;

        const actionOverlayEl = document.getElementById("actionOverlay");
        const actionCanvasEl = document.getElementById("actionCanvas");
        const actionNodesEl = document.getElementById("actionNodes");
        const actionCloseEl = document.getElementById("actionClose");
        const actionLogoEl = document.getElementById("actionLogo");
        const actionTitleEl = document.getElementById("actionTitle");
        const actionSubEl = document.getElementById("actionSub");

        let actionRaf = 0;
        let actionCleanup = null;
        let actionOpen = false;

        let actionTimingEl = null;
        let actionTimingInterval = 0;
        let actionTimingStartMs = 0;
        let actionLastProgress = null;

        let actionProgressWrapEl = null;
        let actionProgressTextEl = null;
        let actionProgressBarOuterEl = null;
        let actionProgressBarInnerEl = null;

        function ensureActionTimingEl() {
          if (actionTimingEl) return actionTimingEl;
          if (!actionSubEl) return null;
          const el = document.createElement("div");
          el.style.marginTop = "10px";
          el.style.opacity = "0.9";
          el.style.fontSize = "13px";
          el.style.lineHeight = "1.2";
          el.style.textAlign = "center";
          el.style.fontFamily = "inherit";
          actionSubEl.insertAdjacentElement("afterend", el);
          actionTimingEl = el;
          return el;
        }

        function ensureActionProgressEls() {
          if (actionProgressWrapEl) return actionProgressWrapEl;
          const timingEl = ensureActionTimingEl();
          if (!timingEl) return null;

          const wrap = document.createElement("div");
          wrap.style.marginTop = "10px";
          wrap.style.display = "flex";
          wrap.style.flexDirection = "column";
          wrap.style.gap = "8px";
          wrap.style.alignItems = "center";
          wrap.style.justifyContent = "center";

          const text = document.createElement("div");
          text.style.opacity = "0.92";
          text.style.fontSize = "13px";
          text.style.lineHeight = "1.2";
          text.style.textAlign = "center";
          text.style.fontFamily = "inherit";

          const outer = document.createElement("div");
          outer.style.width = "min(520px, 72vw)";
          outer.style.height = "10px";
          outer.style.borderRadius = "999px";
          outer.style.background = "rgba(255,255,255,0.10)";
          outer.style.border = "1px solid rgba(255,255,255,0.14)";
          outer.style.overflow = "hidden";

          const inner = document.createElement("div");
          inner.style.height = "100%";
          inner.style.width = "0%";
          inner.style.borderRadius = "999px";
          inner.style.background = "linear-gradient(90deg, rgba(73,234,203,0.95), rgba(73,234,203,0.55))";
          inner.style.boxShadow = "0 0 24px rgba(73,234,203,0.18)";
          inner.style.transition = "width 220ms ease";
          outer.appendChild(inner);

          wrap.appendChild(text);
          wrap.appendChild(outer);

          timingEl.insertAdjacentElement("afterend", wrap);

          actionProgressWrapEl = wrap;
          actionProgressTextEl = text;
          actionProgressBarOuterEl = outer;
          actionProgressBarInnerEl = inner;

          return wrap;
        }

        function formatDurationSec(totalSec) {
          const s = Math.max(0, Math.floor(Number(totalSec) || 0));
          const hh = Math.floor(s / 3600);
          const mm = Math.floor((s % 3600) / 60);
          const ss = s % 60;
          if (hh > 0) return `${hh}:${String(mm).padStart(2, "0")}:${String(ss).padStart(2, "0")}`;
          return `${String(mm).padStart(2, "0")}:${String(ss).padStart(2, "0")}`;
        }

        function computeEtaSec() {
          if (!actionLastProgress) return null;
          const total = Number(actionLastProgress.total || 0);
          const done = Number(actionLastProgress.done || 0);
          const samples = Array.isArray(actionLastProgress.samples) ? actionLastProgress.samples : [];
          if (!Number.isFinite(total) || total <= 0) return null;
          if (!Number.isFinite(done) || done <= 0) return null;
          if (done >= total) return 0;

          const computeFromRate = (rate) => {
            if (!Number.isFinite(rate) || rate <= 0) return null;
            const remaining = total - done;
            return Math.max(0, Math.round(remaining / rate));
          };

          if (samples.length >= 2) {
            const a = samples[0];
            const b = samples[samples.length - 1];
            if (a && b) {
              const dt = (b.t - a.t) / 1000;
              const dd = b.done - a.done;
              if (Number.isFinite(dt) && dt > 0.15 && Number.isFinite(dd) && dd > 0) {
                const eta = computeFromRate(dd / dt);
                if (eta !== null) return eta;
              }
            }
          }

          const now = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();
          const start = Number(actionTimingStartMs || 0);
          const elapsed = start > 0 ? (now - start) / 1000 : 0;
          if (Number.isFinite(elapsed) && elapsed > 0.75) {
            const eta = computeFromRate(done / elapsed);
            if (eta !== null) return eta;
          }

          return null;
        }

        function updateActionTimingUi() {
          if (!actionOpen) return;
          const el = ensureActionTimingEl();
          if (!el) return;

          const now = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();
          const start = actionTimingStartMs || now;
          const elapsedSec = Math.max(0, Math.floor((now - start) / 1000));
          const etaSec = computeEtaSec();

          const elapsedText = formatDurationSec(elapsedSec);
          const etaText = (etaSec === null) ? "—" : formatDurationSec(etaSec);

          el.textContent = `Elapsed: ${elapsedText}  |  ETA: ${etaText}`;

          updateActionProgressUi();
        }

        function updateActionProgressUi() {
          if (!actionOpen) return;
          const wrap = ensureActionProgressEls();
          if (!wrap) return;

          const total = Number(actionLastProgress && actionLastProgress.total);
          const done = Number(actionLastProgress && actionLastProgress.done);
          const kind = actionLastProgress && actionLastProgress.kind;
          if (!Number.isFinite(total) || total <= 0 || !Number.isFinite(done) || done < 0) {
            try {
              if (actionProgressTextEl) actionProgressTextEl.textContent = "Progress: —";
              if (actionProgressBarInnerEl) actionProgressBarInnerEl.style.width = "0%";
            } catch (_) {}
            return;
          }

          const clampedDone = Math.max(0, Math.min(total, done));
          const pct = Math.max(0, Math.min(1, total > 0 ? (clampedDone / total) : 0));
          const pctText = `${Math.round(pct * 100)}%`;
          const label = kind === "receive" ? "Receiving" : "Publishing";

          try {
            if (actionProgressTextEl) actionProgressTextEl.textContent = `${label}: ${clampedDone}/${total} chunks (${pctText})`;
            if (actionProgressBarInnerEl) actionProgressBarInnerEl.style.width = `${(pct * 100).toFixed(2)}%`;
          } catch (_) {}
        }

        function startActionTiming() {
          actionTimingStartMs = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();
          actionLastProgress = null;
          try {
            ensureActionProgressEls();
            if (actionProgressTextEl) actionProgressTextEl.textContent = "Progress: —";
            if (actionProgressBarInnerEl) actionProgressBarInnerEl.style.width = "0%";
          } catch (_) {}
          try {
            if (actionTimingInterval) window.clearInterval(actionTimingInterval);
          } catch (_) {}
          actionTimingInterval = window.setInterval(updateActionTimingUi, 1000);
          updateActionTimingUi();
        }

        function stopActionTiming() {
          try {
            if (actionTimingInterval) window.clearInterval(actionTimingInterval);
          } catch (_) {}
          actionTimingInterval = 0;
        }

        function hideActionOverlay() {
          if (!actionOverlayEl) return;
          actionOpen = false;
          actionOverlayEl.classList.remove("actionOverlayOpen");
          actionOverlayEl.setAttribute("aria-hidden", "true");
          if (actionTitleEl) actionTitleEl.textContent = "";
          if (actionSubEl) actionSubEl.textContent = "";
          if (actionLogoEl) actionLogoEl.removeAttribute("src");
          stopActionTiming();
          try {
            if (actionTimingEl) actionTimingEl.textContent = "";
          } catch (_) {}
          try {
            if (actionProgressTextEl) actionProgressTextEl.textContent = "";
            if (actionProgressBarInnerEl) actionProgressBarInnerEl.style.width = "0%";
          } catch (_) {}
          try {
            if (actionCleanup) actionCleanup();
          } catch (_) {}
          actionCleanup = null;
        }

        function startActionDagAnimation({ logoSrc }) {
          if (!actionOverlayEl || !actionCanvasEl || !actionNodesEl) return () => {};
          const ctx = actionCanvasEl.getContext("2d");
          if (!ctx) return () => {};

          const dpr = Math.max(1, window.devicePixelRatio || 1);
          const nodes = [];
          const edges = [];
          let tips = [];
          let w = 0;
          let h = 0;
          const maxNodes = 22;
          const spawnEveryMs = 260;
          const edgeDrawMs = 520;
          const startAt = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();
          let nextSpawnAt = 0;

          const clamp01 = (x) => Math.max(0, Math.min(1, x));
          const ease = (x) => (x < 0.5 ? 2 * x * x : 1 - Math.pow(-2 * x + 2, 2) / 2);
          const pick = (arr, exclude) => {
            const a = (arr || []).filter((v) => v !== exclude);
            if (a.length === 0) return null;
            return a[Math.floor(Math.random() * a.length)];
          };

          const resize = () => {
            const rect = actionOverlayEl.getBoundingClientRect();
            w = Math.max(1, Math.floor(rect.width));
            h = Math.max(1, Math.floor(rect.height));
            actionCanvasEl.width = Math.floor(w * dpr);
            actionCanvasEl.height = Math.floor(h * dpr);
            actionCanvasEl.style.width = `${w}px`;
            actionCanvasEl.style.height = `${h}px`;
            ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
          };

          const cx = () => w * 0.5;

          const addNode = (x, y) => {
            const el = document.createElement("div");
            el.className = "actionNode";
            const img = document.createElement("img");
            img.className = "actionNodeImg";
            img.alt = "";
            img.src = logoSrc;
            el.appendChild(img);
            actionNodesEl.appendChild(el);

            el.style.opacity = "0";
            el.style.transition = "opacity 420ms ease";

            const n = {
              x,
              y,
              el,
              p: Math.random() * Math.PI * 2,
              ax: 4 + Math.random() * 8,
              ay: 3 + Math.random() * 7,
            };
            nodes.push(n);
            window.requestAnimationFrame(() => {
              el.style.opacity = "0.92";
            });
            return nodes.length - 1;
          };

          const addEdge = (from, to, createdAt) => {
            edges.push({ from, to, createdAt });
          };

          resize();
          window.addEventListener("resize", resize);

          const genesisCount = 3;
          for (let i = 0; i < genesisCount; i++) {
            const gx = cx() + (i - (genesisCount - 1) / 2) * 88 + (Math.random() - 0.5) * 22;
            const gy = h * 0.22 + (Math.random() - 0.5) * 14;
            const idx = addNode(gx, gy);
            tips.push(idx);
          }
          nextSpawnAt = spawnEveryMs;

          const spawnNode = (now) => {
            const i = nodes.length;
            const prog = clamp01((i - genesisCount) / Math.max(1, (maxNodes - genesisCount - 1)));
            const yBase = h * 0.26 + prog * (h * 0.52);
            const xBase = cx() + (Math.random() - 0.5) * (w * 0.52);
            const idx = addNode(xBase, yBase);

            const p1 = pick(tips, null);
            let p2 = null;
            if (Math.random() < 0.65 && tips.length > 1) {
              p2 = pick(tips, p1);
            }

            [p1, p2].filter((p) => p !== null && p !== undefined).forEach((p) => {
              addEdge(p, idx, now);
              tips = tips.filter((t) => t !== p);
            });
            tips.push(idx);
            if (tips.length > 7) tips = tips.slice(tips.length - 7);
          };

          const tick = (t) => {
            ctx.clearRect(0, 0, w, h);
            const now = t;

            while (now - startAt >= nextSpawnAt && nodes.length < maxNodes) {
              spawnNode(now);
              nextSpawnAt += spawnEveryMs;
            }

            for (let i = 0; i < edges.length; i++) {
              const e = edges[i];
              const a = nodes[e.from];
              const b = nodes[e.to];
              if (!a || !b) continue;

              const p = clamp01((now - e.createdAt) / edgeDrawMs);
              const k = ease(p);

              const ajx = Math.sin(now * 0.00115 + a.p) * a.ax;
              const ajy = Math.cos(now * 0.00105 + a.p) * a.ay;
              const bjx = Math.sin(now * 0.00118 + b.p) * b.ax;
              const bjy = Math.cos(now * 0.00108 + b.p) * b.ay;

              const x1 = a.x + ajx;
              const y1 = a.y + ajy;
              const x2 = b.x + bjx;
              const y2 = b.y + bjy;
              const xe = x1 + (x2 - x1) * k;
              const ye = y1 + (y2 - y1) * k;

              ctx.lineWidth = 1.6;
              ctx.strokeStyle = `rgba(73, 234, 203, ${0.07 + k * 0.16})`;
              ctx.beginPath();
              ctx.moveTo(x1, y1);
              ctx.lineTo(xe, ye);
              ctx.stroke();

              ctx.lineWidth = 0.9;
              ctx.strokeStyle = `rgba(112, 199, 186, ${0.05 + k * 0.10})`;
              ctx.beginPath();
              ctx.moveTo(x1, y1);
              ctx.lineTo(xe, ye);
              ctx.stroke();

              ctx.fillStyle = `rgba(73, 234, 203, ${0.08 + k * 0.14})`;
              ctx.beginPath();
              ctx.arc(xe, ye, 2.0, 0, Math.PI * 2);
              ctx.fill();
            }

            for (let i = 0; i < nodes.length; i++) {
              const n = nodes[i];
              const jx = Math.sin(now * 0.00115 + n.p) * n.ax;
              const jy = Math.cos(now * 0.00105 + n.p) * n.ay;
              const x = n.x + jx;
              const y = n.y + jy;
              n.el.style.left = `${x}px`;
              n.el.style.top = `${y}px`;
              n.el.style.animationDelay = `${Math.round(n.p * 100)}ms`;
            }

            actionRaf = window.requestAnimationFrame(tick);
          };

          actionRaf = window.requestAnimationFrame(tick);

          return () => {
            try {
              window.cancelAnimationFrame(actionRaf);
            } catch (_) {}
            try {
              window.removeEventListener("resize", resize);
            } catch (_) {}
            try {
              actionNodesEl.innerHTML = "";
            } catch (_) {}
          };
        }

        function showActionOverlay({ title, sub, theme }) {
          if (!actionOverlayEl) return;
          actionOpen = true;
          actionOverlayEl.classList.add("actionOverlayOpen");
          actionOverlayEl.setAttribute("aria-hidden", "false");

          const logoSrc = theme === "kaspa" ? "assets/kaspa.svg" : "assets/kat.svg";
          if (actionLogoEl) actionLogoEl.src = logoSrc;
          if (actionTitleEl) actionTitleEl.textContent = title || "Working…";
          if (actionSubEl) actionSubEl.textContent = sub || "";

          try {
            if (actionCleanup) actionCleanup();
          } catch (_) {}
          actionCleanup = startActionDagAnimation({ logoSrc });

          startActionTiming();
        }

        if (actionCloseEl) {
          actionCloseEl.addEventListener("click", () => {
            hideActionOverlay();
          });
        }

        function updateSendPrivUi() {
          if (!sendPrivEl) return;
          if (isWalletSessionUnlocked) {
            sendPrivEl.value = "";
            sendPrivEl.readOnly = true;
            sendPrivEl.placeholder = "Wallet unlocked (signing automatically)";
            sendPrivEl.style.opacity = "0.7";
            sendPrivEl.style.cursor = "not-allowed";
            if (sendPrivAutoHintEl) sendPrivAutoHintEl.style.display = "block";
          } else {
            sendPrivEl.readOnly = false;
            sendPrivEl.placeholder = "";
            sendPrivEl.style.opacity = "1";
            sendPrivEl.style.cursor = "text";
            if (sendPrivAutoHintEl) sendPrivAutoHintEl.style.display = "none";
          }
        }

        function updateStudioPrivUi() {
          const apply = (inputEl, hintEl) => {
            if (!inputEl) return;
            if (isWalletSessionUnlocked) {
              inputEl.value = "";
              inputEl.readOnly = true;
              inputEl.placeholder = "Wallet unlocked (signing automatically)";
              inputEl.style.opacity = "0.7";
              inputEl.style.cursor = "not-allowed";
              if (hintEl) hintEl.style.display = "block";
            } else {
              inputEl.readOnly = false;
              inputEl.placeholder = "From private key (hex)";
              inputEl.style.opacity = "1";
              inputEl.style.cursor = "text";
              if (hintEl) hintEl.style.display = "none";
            }
          };

          apply(studioAudioPrivEl, studioAudioPrivAutoHintEl);
          apply(studioVideoPrivEl, studioVideoPrivAutoHintEl);
        }

        function walletNetworkToKnsNetwork(walletNetwork) {
          const n = String(walletNetwork || "").trim().toLowerCase();
          if (!n || n === "mainnet") return "mainnet";
          if (n === "testnet") return "tn10";
          if (n === "tn10" || n === "testnet10") return "tn10";
          return null;
        }

        function knsNetworkFromRpcUrl(rpcUrl) {
          const s = String(rpcUrl || "").trim().toLowerCase();
          if (!s) return "mainnet";
          if (s.includes("testnet") || s.includes("tn10") || s.includes(":16210")) return "tn10";
          return "mainnet";
        }

        function isKnsNameCandidate(input) {
          const s = String(input || "").trim().toLowerCase();
          return !!s && !s.includes(":") && s.endsWith(".kas");
        }

        function clearKnsPreview(previewEl) {
          if (!previewEl) return;
          previewEl.style.display = "none";
          previewEl.className = "knsPreview";
          previewEl.textContent = "";
          delete previewEl.dataset.knsInput;
          delete previewEl.dataset.knsNetwork;
          delete previewEl.dataset.resolvedAddr;
        }

        function setKnsPreviewResolving(previewEl) {
          if (!previewEl) return;
          previewEl.style.display = "block";
          previewEl.className = "knsPreview";
          previewEl.textContent = "Resolving…";
        }

        function setKnsPreviewResolved(previewEl, input, knsNetwork, resolvedAddr) {
          if (!previewEl) return;
          previewEl.style.display = "block";
          previewEl.className = "knsPreview";
          previewEl.dataset.knsInput = String(input || "").trim();
          previewEl.dataset.knsNetwork = String(knsNetwork || "");
          previewEl.dataset.resolvedAddr = String(resolvedAddr || "").trim();
          previewEl.innerHTML = `Resolves to: <span class="knsPreviewAddr">${String(resolvedAddr || "").trim()}</span>`;
        }

        function setKnsPreviewError(previewEl, msg) {
          if (!previewEl) return;
          previewEl.style.display = "block";
          previewEl.className = "knsPreview knsPreviewError";
          previewEl.textContent = String(msg || "KNS resolution failed");
          delete previewEl.dataset.resolvedAddr;
        }

        function getCachedResolved(previewEl, input, knsNetwork) {
          if (!previewEl) return null;
          const in0 = String(previewEl.dataset.knsInput || "");
          const net0 = String(previewEl.dataset.knsNetwork || "");
          const addr = String(previewEl.dataset.resolvedAddr || "");
          if (!addr) return null;
          if (in0 !== String(input || "").trim()) return null;
          if (net0 !== String(knsNetwork || "")) return null;
          return addr;
        }

        async function resolveToAddressMaybeKns(input, knsNetwork) {
          const raw = String(input || "").trim();
          if (!raw) return "";
          if (raw.includes(":")) return raw;

          const owner = await tauri.invoke("kns_domain_owner", {
            domain: raw,
            network: knsNetwork || null,
          });
          const out = String(owner || "").trim();
          if (!out) throw new Error("KNS resolution returned empty address");
          return out;
        }

        function debounce(fn, ms) {
          let t = null;
          return (...args) => {
            if (t) clearTimeout(t);
            t = setTimeout(() => {
              t = null;
              fn(...args);
            }, ms);
          };
        }

        function attachKnsPreview({ inputEl, previewEl, getKnsNetwork }) {
          if (!inputEl || !previewEl) return;

          let seq = 0;

          const run = async () => {
            const raw = String(inputEl.value || "").trim();
            const knsNetwork = getKnsNetwork ? getKnsNetwork() : null;

            clearKnsPreview(previewEl);
            if (!isKnsNameCandidate(raw)) return;

            const mySeq = ++seq;
            setKnsPreviewResolving(previewEl);
            try {
              const resolved = await resolveToAddressMaybeKns(raw, knsNetwork);
              if (mySeq !== seq) return;
              if (!resolved) throw new Error("KNS resolution returned empty address");
              setKnsPreviewResolved(previewEl, raw, knsNetwork, resolved);
            } catch (e) {
              if (mySeq !== seq) return;
              setKnsPreviewError(previewEl, `KNS: ${String(e)}`);
            }
          };

          const runDebounced = debounce(run, 450);
          inputEl.addEventListener("input", runDebounced);
          inputEl.addEventListener("blur", run);

          if (getKnsNetwork) {
            const onNetChange = () => runDebounced();
            if (sendRpcEl && inputEl === sendToEl) sendRpcEl.addEventListener("input", onNetChange);
            if (studioAudioRpcEl && inputEl === studioAudioToEl) studioAudioRpcEl.addEventListener("input", onNetChange);
            if (studioVideoRpcEl && inputEl === studioVideoToEl) studioVideoRpcEl.addEventListener("input", onNetChange);
            if (walletNetworkEl && inputEl === walletSendToEl) walletNetworkEl.addEventListener("change", onNetChange);
          }
        }

        if (!tauri?.invoke) {
          setError("Tauri API not available. Are you running the desktop app through Tauri?");
          return;
        }

        if (eventApi?.listen) {
          try {
            await eventApi.listen("tauri://file-drop", (e) => {
              try {
                const payload = e && e.payload;
                const paths = Array.isArray(payload) ? payload : [];
                if (paths.length > 0) {
                  setPickedFilePath(String(paths[0] || ""));
                }
              } catch (_) {
                // ignore
              }
            });
          } catch (_) {
            // ignore
          }
        }

        if (eventApi?.listen) {
          try {
            await eventApi.listen("kaspa_send_progress", (e) => {
              try {
                const p = e && e.payload;
                const done = Number(p && p.submitted_chunks);
                const total = Number(p && p.total_chunks);
                if (!Number.isFinite(done) || done < 0) return;
                if (!Number.isFinite(total) || total <= 0) return;
                const now = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();

                if (!actionLastProgress || actionLastProgress.kind !== "send") {
                  actionLastProgress = { kind: "send", done, total, samples: [{ t: now, done }] };
                } else {
                  actionLastProgress.done = done;
                  actionLastProgress.total = total;
                  const s = Array.isArray(actionLastProgress.samples) ? actionLastProgress.samples : [];
                  s.push({ t: now, done });
                  while (s.length > 6) s.shift();
                  actionLastProgress.samples = s;
                }
                updateActionTimingUi();
              } catch (_) {
                // ignore
              }
            });
          } catch (_) {
            // ignore
          }

          try {
            await eventApi.listen("kaspa_receive_progress", (e) => {
              try {
                const p = e && e.payload;
                const done = Number(p && p.found_chunks);
                const total = Number(p && p.total_chunks);
                if (!Number.isFinite(done) || done < 0) return;
                if (!Number.isFinite(total) || total <= 0) return;
                const now = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();

                if (!actionLastProgress || actionLastProgress.kind !== "receive") {
                  actionLastProgress = { kind: "receive", done, total, samples: [{ t: now, done }] };
                } else {
                  actionLastProgress.done = done;
                  actionLastProgress.total = total;
                  const s = Array.isArray(actionLastProgress.samples) ? actionLastProgress.samples : [];
                  s.push({ t: now, done });
                  while (s.length > 6) s.shift();
                  actionLastProgress.samples = s;
                }
                updateActionTimingUi();
              } catch (_) {
                // ignore
              }
            });
          } catch (_) {
            // ignore
          }
        }

        async function refreshNodeBundleDir() {
          try {
            const d = await tauri.invoke("node_bundle_get_dir", {});
            nodeBundleDir = String(d || "");
          } catch (_) {
            nodeBundleDir = "";
          }

          if (nodeBundleDirOutEl) nodeBundleDirOutEl.textContent = nodeBundleDir || "—";
          if (nodeBundleOpenBtnEl) nodeBundleOpenBtnEl.disabled = !nodeBundleDir;
          if (nodeBundleOpenBtnEl) nodeBundleOpenBtnEl.style.opacity = nodeBundleDir ? "1" : "0.6";
          if (nodeBundleOpenBtnEl) nodeBundleOpenBtnEl.style.cursor = nodeBundleDir ? "pointer" : "not-allowed";
        }

        attachKnsPreview({
          inputEl: sendToEl,
          previewEl: sendToResolvedEl,
          getKnsNetwork: () => knsNetworkFromRpcUrl(sendRpcEl?.value || ""),
        });
        attachKnsPreview({
          inputEl: walletSendToEl,
          previewEl: walletSendToResolvedEl,
          getKnsNetwork: () => walletNetworkToKnsNetwork(walletNetworkEl?.value || ""),
        });
        attachKnsPreview({
          inputEl: studioAudioToEl,
          previewEl: studioAudioToResolvedEl,
          getKnsNetwork: () => knsNetworkFromRpcUrl(studioAudioRpcEl?.value || ""),
        });
        attachKnsPreview({
          inputEl: studioVideoToEl,
          previewEl: studioVideoToResolvedEl,
          getKnsNetwork: () => knsNetworkFromRpcUrl(studioVideoRpcEl?.value || ""),
        });

        const globalRpcModeLocalEl = document.getElementById("globalRpcModeLocal");
        const globalRpcModePublicEl = document.getElementById("globalRpcModePublic");

        const sendRpcModeLocalEl = document.getElementById("sendRpcModeLocal");
        const sendRpcModePublicEl = document.getElementById("sendRpcModePublic");
        const recvRpcModeLocalEl = document.getElementById("recvRpcModeLocal");
        const recvRpcModePublicEl = document.getElementById("recvRpcModePublic");
        const studioAudioRpcModeLocalEl = document.getElementById("studioAudioRpcModeLocal");
        const studioAudioRpcModePublicEl = document.getElementById("studioAudioRpcModePublic");
        const studioVideoRpcModeLocalEl = document.getElementById("studioVideoRpcModeLocal");
        const studioVideoRpcModePublicEl = document.getElementById("studioVideoRpcModePublic");

        function isPublicResolverValue(v) {
          const s = String(v || "").trim().toLowerCase();
          return s === "public" || s === "resolver" || s.startsWith("public:") || s.startsWith("resolver:");
        }

        function setToggleActive(localBtn, publicBtn, isPublic) {
          if (!localBtn || !publicBtn) return;
          if (isPublic) {
            localBtn.classList.remove("tabBtnActive");
            publicBtn.classList.add("tabBtnActive");
          } else {
            publicBtn.classList.remove("tabBtnActive");
            localBtn.classList.add("tabBtnActive");
          }
        }

        function getGlobalRpcMode() {
          try {
            const v = String(localStorage.getItem("kat_rpc_mode") || "").trim().toLowerCase();
            if (v === "public") return "public";
          } catch (_) {}
          return "local";
        }

        function publicValueForWallet() {
          const n = String(walletNetworkEl?.value || "mainnet").toLowerCase();
          if (n.includes("testnet")) return "public:tn10";
          if (n.includes("devnet")) return "public:devnet";
          return "public";
        }

        function applyGlobalRpcMode(mode) {
          const m = mode === "public" ? "public" : "local";
          setToggleActive(globalRpcModeLocalEl, globalRpcModePublicEl, m === "public");

          const applyToInput = (inputEl, publicValue) => {
            if (!inputEl) return;
            const current = String(inputEl.value || "").trim();
            const isPub = isPublicResolverValue(current);
            if (m === "public") {
              inputEl.value = publicValue;
            } else {
              inputEl.value = isPub || !current ? "grpc://127.0.0.1:16110" : current;
            }
            inputEl.dispatchEvent(new Event("input", { bubbles: true }));
          };

          applyToInput(sendRpcEl, "public");
          applyToInput(recvRpcEl, "public");
          applyToInput(studioAudioRpcEl, "public");
          applyToInput(studioVideoRpcEl, "public");
          applyToInput(walletRpcUrlEl, publicValueForWallet());
        }

        function setGlobalRpcMode(mode) {
          const m = mode === "public" ? "public" : "local";
          try {
            localStorage.setItem("kat_rpc_mode", m);
          } catch (_) {}
          applyGlobalRpcMode(m);
        }

        function setupRpcModeToggle({ inputEl, localBtn, publicBtn, getPublicValue, getLocalDefaultValue }) {
          if (!inputEl || !localBtn || !publicBtn) return;
          let lastLocal = String(inputEl.value || "").trim() || (getLocalDefaultValue ? getLocalDefaultValue() : "grpc://127.0.0.1:16110");

          const refresh = () => {
            setToggleActive(localBtn, publicBtn, isPublicResolverValue(inputEl.value));
          };

          localBtn.addEventListener("click", () => {
            const current = String(inputEl.value || "").trim();
            if (!isPublicResolverValue(current) && current) lastLocal = current;
            inputEl.value = lastLocal || (getLocalDefaultValue ? getLocalDefaultValue() : "grpc://127.0.0.1:16110");
            inputEl.dispatchEvent(new Event("input", { bubbles: true }));
            refresh();
          });

          publicBtn.addEventListener("click", () => {
            const current = String(inputEl.value || "").trim();
            if (!isPublicResolverValue(current) && current) lastLocal = current;
            inputEl.value = getPublicValue ? getPublicValue() : "public";
            inputEl.dispatchEvent(new Event("input", { bubbles: true }));
            refresh();
          });

          inputEl.addEventListener("input", refresh);
          refresh();
        }

        setupRpcModeToggle({
          inputEl: sendRpcEl,
          localBtn: sendRpcModeLocalEl,
          publicBtn: sendRpcModePublicEl,
          getPublicValue: () => "public",
          getLocalDefaultValue: () => "grpc://127.0.0.1:16110",
        });
        setupRpcModeToggle({
          inputEl: recvRpcEl,
          localBtn: recvRpcModeLocalEl,
          publicBtn: recvRpcModePublicEl,
          getPublicValue: () => "public",
          getLocalDefaultValue: () => "grpc://127.0.0.1:16110",
        });
        setupRpcModeToggle({
          inputEl: studioAudioRpcEl,
          localBtn: studioAudioRpcModeLocalEl,
          publicBtn: studioAudioRpcModePublicEl,
          getPublicValue: () => "public",
          getLocalDefaultValue: () => "grpc://127.0.0.1:16110",
        });
        setupRpcModeToggle({
          inputEl: studioVideoRpcEl,
          localBtn: studioVideoRpcModeLocalEl,
          publicBtn: studioVideoRpcModePublicEl,
          getPublicValue: () => "public",
          getLocalDefaultValue: () => "grpc://127.0.0.1:16110",
        });
        setupRpcModeToggle({
          inputEl: walletRpcUrlEl,
          localBtn: walletRpcModeLocalEl,
          publicBtn: walletRpcModePublicEl,
          getPublicValue: () => {
            const n = String(walletNetworkEl?.value || "mainnet").toLowerCase();
            if (n.includes("testnet")) return "public:tn10";
            if (n.includes("devnet")) return "public:devnet";
            return "public";
          },
          getLocalDefaultValue: () => "grpc://127.0.0.1:16110",
        });

        if (walletNetworkEl && walletRpcUrlEl) {
          walletNetworkEl.addEventListener("change", () => {
            if (!isPublicResolverValue(walletRpcUrlEl.value)) return;
            const n = String(walletNetworkEl.value || "mainnet").toLowerCase();
            if (n.includes("testnet")) walletRpcUrlEl.value = "public:tn10";
            else if (n.includes("devnet")) walletRpcUrlEl.value = "public:devnet";
            else walletRpcUrlEl.value = "public";
            walletRpcUrlEl.dispatchEvent(new Event("input", { bubbles: true }));
          });
        }

        if (globalRpcModeLocalEl && globalRpcModePublicEl) {
          globalRpcModeLocalEl.addEventListener("click", () => setGlobalRpcMode("local"));
          globalRpcModePublicEl.addEventListener("click", () => setGlobalRpcMode("public"));
          applyGlobalRpcMode(getGlobalRpcMode());
        }

        let rpcConnTimer = null;
        async function refreshRpcConnInfo() {
          if (!rpcConnTextEl) return;
          try {
            const network = String(walletNetworkEl?.value || "mainnet");
            const rpcUrl = String(walletRpcUrlEl?.value || "").trim() || null;
            const info = await tauri.invoke("rpc_connection_info", { network, rpcUrl });

            const mode = isPublicResolverValue(info?.rpcUrl) ? "Public" : "Local";
            const net = String(info?.network || network);
            const synced = info?.isSynced ? "synced" : "syncing";
            const utxo = info?.isUtxoIndexed ? "utxo" : "no-utxo";
            const p2p = String(info?.p2pId || "").trim();
            const p2pShort = p2p ? `${p2p.slice(0, 10)}…` : "";

            rpcConnTextEl.textContent = `${mode} ${net} • ${synced} • ${utxo}${p2pShort ? ` • ${p2pShort}` : ""}`;
          } catch (_) {
            rpcConnTextEl.textContent = "Disconnected";
          }
        }

        if (rpcConnTextEl) {
          refreshRpcConnInfo();
          if (rpcConnTimer) clearInterval(rpcConnTimer);
          rpcConnTimer = setInterval(refreshRpcConnInfo, 4000);
          if (walletRpcUrlEl) walletRpcUrlEl.addEventListener("input", () => refreshRpcConnInfo());
          if (walletNetworkEl) walletNetworkEl.addEventListener("change", () => refreshRpcConnInfo());
        }

        try {
          document.querySelectorAll(".rpcMode").forEach((el) => {
            el.style.display = "none";
          });
        } catch (_) {}

        if (modalCloseEl) modalCloseEl.addEventListener("click", closeModal);
        if (modalOverlayEl) {
          modalOverlayEl.addEventListener("click", (e) => {
            if (e.target === modalOverlayEl) closeModal();
          });
        }
        window.addEventListener("keydown", (e) => {
          if (e.key === "Escape") closeModal();
        });

        if (clearSessionBtnEl) {
          clearSessionBtnEl.addEventListener("click", () => {
            clearSession();
          });
        }

        if (nodeBundleImportBtnEl) {
          nodeBundleImportBtnEl.addEventListener("click", async () => {
            setError("");
            try {
              const picked = await dialogApi.open({ directory: true, multiple: false });
              if (!picked) return;
              await tauri.invoke("node_bundle_set_dir", { dir: String(picked) });
              await refreshNodeBundleDir();
              showModal({ title: "Imported", body: `Node tools folder set to:\n${nodeBundleDir}`, actions: [{ label: "OK", primary: true }] });
            } catch (e) {
              setError(String(e));
              showModal({ title: "Import failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (nodeBundleOpenBtnEl) {
          nodeBundleOpenBtnEl.addEventListener("click", async () => {
            setError("");
            try {
              if (!nodeBundleDir) return;
              await tauri.invoke("open_file", { path: nodeBundleDir });
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (nodeBundleClearBtnEl) {
          nodeBundleClearBtnEl.addEventListener("click", async () => {
            showModal({
              title: "Clear imported folder?",
              body: "This will remove the saved node tools folder path from this device.",
              actions: [
                {
                  label: "Clear",
                  primary: true,
                  onClick: async () => {
                    await tauri.invoke("node_bundle_set_dir", { dir: null });
                    await refreshNodeBundleDir();
                  },
                },
                { label: "Cancel" },
              ],
            });
          });
        }

        function setWalletTab(name) {
          const isOverview = name === "overview";
          const isManage = name === "manage";
          const isAdvanced = name === "advanced";
          if (walletTabOverviewEl) walletTabOverviewEl.className = `tabBtn ${isOverview ? "tabBtnActive" : ""}`.trim();
          if (walletTabManageEl) walletTabManageEl.className = `tabBtn ${isManage ? "tabBtnActive" : ""}`.trim();
          if (walletTabAdvancedEl) walletTabAdvancedEl.className = `tabBtn ${isAdvanced ? "tabBtnActive" : ""}`.trim();
          if (walletOverviewPanelEl) walletOverviewPanelEl.style.display = isOverview ? "block" : "none";
          if (walletManagePanelEl) walletManagePanelEl.style.display = isManage ? "block" : "none";
          if (walletAdvancedPanelEl) walletAdvancedPanelEl.style.display = isAdvanced ? "block" : "none";

          if (walletShellEl) walletShellEl.classList.toggle("walletShellOverview", isOverview);
        }

        function setStudioTab(name) {
          const isAudio = name === "audio";
          const isVideo = name === "video";

          if (studioTabAudioEl) studioTabAudioEl.className = `tabBtn ${isAudio ? "tabBtnActive" : ""}`.trim();
          if (studioTabVideoEl) studioTabVideoEl.className = `tabBtn ${isVideo ? "tabBtnActive" : ""}`.trim();
          if (studioAudioPanelEl) studioAudioPanelEl.style.display = isAudio ? "block" : "none";
          if (studioVideoPanelEl) studioVideoPanelEl.style.display = isVideo ? "block" : "none";
        }

        function syncWalletHeroBalance() {
          if (!walletHeroBalanceEl || !walletBalanceEl) return;
          const t = String(walletBalanceEl.textContent || "—").trim();
          walletHeroBalanceEl.textContent = t || "—";
        }

        function openWalletManageSection(sectionEl) {
          setWalletTab("manage");
          try {
            if (sectionEl && typeof sectionEl.scrollIntoView === "function") {
              sectionEl.scrollIntoView({ behavior: "smooth", block: "start" });
            }
          } catch (_) {
            // ignore
          }
        }

        if (walletTabOverviewEl) walletTabOverviewEl.addEventListener("click", () => {
          setWalletTab("overview");
          syncWalletHeroBalance();
        });
        if (walletTabManageEl) walletTabManageEl.addEventListener("click", () => setWalletTab("manage"));
        if (walletTabAdvancedEl) walletTabAdvancedEl.addEventListener("click", () => setWalletTab("advanced"));

        if (studioTabAudioEl) studioTabAudioEl.addEventListener("click", () => setStudioTab("audio"));
        if (studioTabVideoEl) studioTabVideoEl.addEventListener("click", () => setStudioTab("video"));

        if (walletHdrCreateEl) walletHdrCreateEl.addEventListener("click", () => openWalletManageSection(walletManageCreateSectionEl));
        if (walletHdrImportEl) walletHdrImportEl.addEventListener("click", () => openWalletManageSection(walletManageImportSectionEl));
        if (walletHdrDeleteEl) walletHdrDeleteEl.addEventListener("click", () => openWalletManageSection(walletManageDataSectionEl));

        setWalletTab("overview");
        syncWalletHeroBalance();

        setStudioTab("audio");

        async function refreshWalletStatus() {
          try {
            const u = await tauri.invoke("wallet_unlocked_username", {});
            if (u) {
              isWalletSessionUnlocked = true;
              updateSendPrivUi();
              updateStudioPrivUi();
              walletStatusEl.textContent = "Unlocked";
              walletUnlockedEl.textContent = String(u);
            } else {
              isWalletSessionUnlocked = false;
              updateSendPrivUi();
              updateStudioPrivUi();
              walletStatusEl.textContent = "Locked";
              walletUnlockedEl.textContent = "—";
              if (walletFullPathEl) walletFullPathEl.textContent = "—";
              if (walletDerivedAddressEl) walletDerivedAddressEl.textContent = "—";
              if (walletBalanceEl) walletBalanceEl.textContent = "—";
              if (walletSendKasTxidEl) walletSendKasTxidEl.textContent = "—";
              if (walletTxHistoryStatusEl) walletTxHistoryStatusEl.textContent = "Unlock wallet to view history.";
              if (walletTxReceivedListEl) walletTxReceivedListEl.innerHTML = "";
              if (walletTxSentListEl) walletTxSentListEl.innerHTML = "";
            }
          } catch (_) {
            isWalletSessionUnlocked = false;
            updateSendPrivUi();
            updateStudioPrivUi();
            walletStatusEl.textContent = "Wallet error";
            walletUnlockedEl.textContent = "—";
          }
        }

        async function refreshWalletTxHistory() {
          if (!walletTxHistoryStatusEl || !walletTxReceivedListEl || !walletTxSentListEl) return;
          if (!isWalletSessionUnlocked) {
            walletTxHistoryStatusEl.textContent = "Unlock wallet to view history.";
            walletTxReceivedListEl.innerHTML = "";
            walletTxSentListEl.innerHTML = "";
            return;
          }

          const address = String(walletDerivedAddressEl?.textContent || "").trim();
          if (!address || address === "—") {
            walletTxHistoryStatusEl.textContent = "Derive an address to view history.";
            walletTxReceivedListEl.innerHTML = "";
            walletTxSentListEl.innerHTML = "";
            return;
          }

          walletTxHistoryStatusEl.textContent = "Loading…";
          walletTxReceivedListEl.innerHTML = "";
          walletTxSentListEl.innerHTML = "";

          const network = walletNetworkEl?.value || "mainnet";

          const items = (await tauri.invoke("wallet_tx_history", {
            network,
            address,
            limit: 50,
            offset: 0,
          })) || [];

          const received = [];
          const sent = [];

          (items || []).forEach((it) => {
            const netSompi = Number(it?.netSompi || 0);
            if (netSompi > 0) received.push(it);
            else if (netSompi < 0) sent.push(it);
          });

          const mkRow = (it, kind) => {
            const txid = String(it?.txid || "").trim();
            const netSompi = Number(it?.netSompi || 0);
            const ts = it?.timestampMs ? new Date(Number(it.timestampMs)) : null;
            const when = ts && Number.isFinite(ts.getTime()) ? ts.toLocaleString() : "";
            const accepted = it?.accepted;
            const accText = accepted === true ? "Accepted" : accepted === false ? "Pending" : "";
            const meta = [when, accText].filter(Boolean).join(" • ");

            const row = document.createElement("div");
            row.className = "txRow";

            const badge = document.createElement("div");
            badge.className = `txBadge ${kind === "sent" ? "txBadgeSent" : "txBadgeRecv"}`.trim();
            badge.textContent = kind === "sent" ? "Sent" : "Received";

            const main = document.createElement("div");
            main.className = "txMain";

            const id = document.createElement("div");
            id.className = "txId";
            id.textContent = txid || "(unknown txid)";

            const metaEl = document.createElement("div");
            metaEl.className = "txMeta";
            metaEl.textContent = meta;

            main.appendChild(id);
            if (meta) main.appendChild(metaEl);

            const amt = document.createElement("div");
            amt.className = "txAmt";
            const absSompi = Math.abs(netSompi);
            const sign = kind === "sent" ? "-" : "+";
            amt.textContent = `${sign}${formatKasFromSompi(absSompi)} KAS`;

            row.appendChild(badge);
            row.appendChild(main);
            row.appendChild(amt);

            if (txid) {
              row.style.cursor = "pointer";
              row.addEventListener("click", () => openExternal(getExplorerTxUrl(txid)));
            }
            return row;
          };

          if (received.length === 0 && sent.length === 0) {
            walletTxHistoryStatusEl.textContent = "No transactions found for this address.";
            return;
          }

          walletTxHistoryStatusEl.textContent = "";

          if (received.length === 0) {
            walletTxReceivedListEl.textContent = "No received transactions.";
          } else {
            received.slice(0, 25).forEach((it) => walletTxReceivedListEl.appendChild(mkRow(it, "recv")));
          }

          if (sent.length === 0) {
            walletTxSentListEl.textContent = "No sent transactions.";
          } else {
            sent.slice(0, 25).forEach((it) => walletTxSentListEl.appendChild(mkRow(it, "sent")));
          }
        }

        if (walletTxRefreshBtnEl) {
          walletTxRefreshBtnEl.addEventListener("click", async () => {
            setError("");
            try {
              await refreshWalletTxHistory();
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (sendPrivEl) {
          const showAutoSignModal = () => {
            if (!isWalletSessionUnlocked) return;
            showModal({
              title: "Signing automatically",
              body: "Your wallet is unlocked, so transfers will be signed automatically using the active wallet key.\n\nTo use a manual private key instead, lock the wallet.",
              actions: [
                { label: "OK", primary: true },
                {
                  label: "Lock wallet",
                  onClick: async () => {
                    await tauri.invoke("wallet_lock", {});
                    walletPasswordEl.value = "";
                    walletProfileSelectEl.value = "";
                    await refreshWalletStatus();
                  },
                },
              ],
            });
          };

          sendPrivEl.addEventListener("click", showAutoSignModal);
          sendPrivEl.addEventListener("focus", showAutoSignModal);
        }

        const showAutoSignModalStudio = async () => {
          if (!isWalletSessionUnlocked) return;
          showModal({
            title: "Signing automatically",
            body: "Your wallet is unlocked, so Studio sends will be signed automatically using the active wallet key.\n\nTo use a manual private key instead, lock the wallet.",
            actions: [
              { label: "OK", primary: true },
              {
                label: "Lock wallet",
                onClick: async () => {
                  await tauri.invoke("wallet_lock", {});
                  walletPasswordEl.value = "";
                  walletProfileSelectEl.value = "";
                  await refreshWalletStatus();
                },
              },
            ],
          });
        };

        if (studioAudioPrivEl) {
          studioAudioPrivEl.addEventListener("click", showAutoSignModalStudio);
          studioAudioPrivEl.addEventListener("focus", showAutoSignModalStudio);
        }
        if (studioVideoPrivEl) {
          studioVideoPrivEl.addEventListener("click", showAutoSignModalStudio);
          studioVideoPrivEl.addEventListener("focus", showAutoSignModalStudio);
        }

        function pickFirstSupportedMime(candidates) {
          for (const t of candidates) {
            try {
              if (window.MediaRecorder && MediaRecorder.isTypeSupported(t)) return t;
            } catch (_) {
              // ignore
            }
          }
          return "";
        }

        async function readBlobDataUrl(blob) {
          return new Promise((resolve, reject) => {
            const r = new FileReader();
            r.onerror = () => reject(new Error("failed reading blob"));

            r.onload = () => resolve(String(r.result || ""));
            r.readAsDataURL(blob);
          });
        }

        if (studioAudioExportMp3El) {
          studioAudioExportMp3El.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioAudioBlob) {
                showModal({ title: "Nothing to export", body: "Record audio first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const ts = new Date().toISOString().replace(/[:.]/g, "-");
              const extIn = (studioAudioBlob.type || "").includes("ogg") ? "ogg" : "webm";
              const inName = `kat_audio_${ts}.${extIn}`;
              if (!studioAudioFilePath) {
                studioAudioFilePath = await ensureStudioTempFilePath(studioAudioBlob, inName);
              }
              const outPath = await dialogApi.save({
                defaultPath: `kat_audio_${ts}.mp3`,
                filters: [{ name: "MP3", extensions: ["mp3"] }],
              });
              if (!outPath) return;
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Exporting…";
              await tauri.invoke("ffmpeg_transcode", {
                inputPath: studioAudioFilePath,
                outputPath: String(outPath),
                kind: "mp3",
              });
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Ready";
              showModal({ title: "Export complete", body: `Saved to:\n${String(outPath)}`, actions: [{ label: "OK", primary: true }] });
            } catch (e) {
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Ready";
              setError(String(e));
              showModal({ title: "Export failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioAudioExportWavEl) {
          studioAudioExportWavEl.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioAudioBlob) {
                showModal({ title: "Nothing to export", body: "Record audio first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const ts = new Date().toISOString().replace(/[:.]/g, "-");
              const extIn = (studioAudioBlob.type || "").includes("ogg") ? "ogg" : "webm";
              const inName = `kat_audio_${ts}.${extIn}`;
              if (!studioAudioFilePath) {
                studioAudioFilePath = await ensureStudioTempFilePath(studioAudioBlob, inName);
              }
              const outPath = await dialogApi.save({
                defaultPath: `kat_audio_${ts}.wav`,
                filters: [{ name: "WAV", extensions: ["wav"] }],
              });
              if (!outPath) return;
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Exporting…";
              await tauri.invoke("ffmpeg_transcode", {
                inputPath: studioAudioFilePath,
                outputPath: String(outPath),
                kind: "wav",
              });
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Ready";
              showModal({ title: "Export complete", body: `Saved to:\n${String(outPath)}`, actions: [{ label: "OK", primary: true }] });
            } catch (e) {
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Ready";
              setError(String(e));
              showModal({ title: "Export failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        async function saveBlobToFile(blob, suggestedName, filters) {
          if (!dialogApi?.save || !fsApi?.writeFile) {
            throw new Error("Save is not available (missing dialog/fs API)");
          }
          const path = await dialogApi.save({
            defaultPath: suggestedName,
            filters: filters || [],
          });
          if (!path) return null;
          const buf = await blob.arrayBuffer();
          const bytes = new Uint8Array(buf);
          await fsApi.writeFile(String(path), bytes);
          return String(path);
        }

        async function writeBlobToPath(blob, path) {
          if (!fsApi?.writeFile) {
            throw new Error("File write is not available (missing fs API)");
          }
          const buf = await blob.arrayBuffer();
          const bytes = new Uint8Array(buf);
          await fsApi.writeFile(String(path), bytes);
        }

        async function ensureStudioTempFilePath(blob, fileName) {
          if (!blob) throw new Error("missing blob");
          const tmpPath = await tauri.invoke("studio_temp_path", { fileName });
          await writeBlobToPath(blob, String(tmpPath));
          return String(tmpPath);
        }

        async function listMediaDevices() {
          if (!navigator.mediaDevices?.enumerateDevices) return { mics: [], cams: [] };
          const devices = await navigator.mediaDevices.enumerateDevices();
          const mics = devices.filter((d) => d.kind === "audioinput");
          const cams = devices.filter((d) => d.kind === "videoinput");
          return { mics, cams };
        }

        async function refreshStudioDevices() {
          try {
            if (!navigator.mediaDevices?.getUserMedia) return;
            const tmp = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
            tmp.getTracks().forEach((t) => t.stop());
          } catch (_) {
            // ignore
          }
          const { mics, cams } = await listMediaDevices();
          if (studioAudioMicEl) {
            studioAudioMicEl.innerHTML = "";
            mics.forEach((d) => {
              const o = document.createElement("option");
              o.value = d.deviceId;
              o.textContent = d.label || `Microphone ${d.deviceId.slice(0, 6)}`;
              studioAudioMicEl.appendChild(o);
            });
          }
          if (studioVideoMicEl) {
            studioVideoMicEl.innerHTML = "";
            mics.forEach((d) => {
              const o = document.createElement("option");
              o.value = d.deviceId;
              o.textContent = d.label || `Microphone ${d.deviceId.slice(0, 6)}`;
              studioVideoMicEl.appendChild(o);
            });
          }
          if (studioVideoCamEl) {
            studioVideoCamEl.innerHTML = "";
            cams.forEach((d) => {
              const o = document.createElement("option");
              o.value = d.deviceId;
              o.textContent = d.label || `Camera ${d.deviceId.slice(0, 6)}`;
              studioVideoCamEl.appendChild(o);
            });
          }
        }

        let studioAudioRecorder = null;
        let studioAudioChunks = [];
        let studioAudioBlob = null;
        let studioAudioMime = "";
        let studioAudioStream = null;
        let studioAudioFilePath = "";

        function clearStudioAudioTake() {
          studioAudioBlob = null;
          studioAudioChunks = [];
          studioAudioFilePath = "";
          if (studioAudioPlaybackEl) {
            try {
              if (studioAudioPlaybackEl.src) URL.revokeObjectURL(studioAudioPlaybackEl.src);
            } catch (_) {
              // ignore
            }
            studioAudioPlaybackEl.src = "";
            studioAudioPlaybackEl.style.display = "none";
          }
          if (studioAudioMetaEl) {
            studioAudioMetaEl.style.display = "none";
            studioAudioMetaEl.textContent = "";
          }
          if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Idle";
        }

        async function startStudioAudio() {
          if (!navigator.mediaDevices?.getUserMedia) {
            throw new Error("Audio recording not supported");
          }
          clearStudioAudioTake();

          const micId = studioAudioMicEl?.value || "";
          const constraints = {
            audio: micId ? { deviceId: { exact: micId } } : true,
            video: false,
          };
          studioAudioStream = await navigator.mediaDevices.getUserMedia(constraints);
          studioAudioMime = pickFirstSupportedMime(["audio/webm;codecs=opus", "audio/webm", "audio/ogg;codecs=opus", "audio/ogg"]);
          const opts = studioAudioMime ? { mimeType: studioAudioMime } : undefined;

          studioAudioChunks = [];
          studioAudioRecorder = new MediaRecorder(studioAudioStream, opts);
          studioAudioRecorder.ondataavailable = (e) => {
            if (e.data && e.data.size > 0) studioAudioChunks.push(e.data);
          };
          studioAudioRecorder.onstop = () => {
            studioAudioBlob = new Blob(studioAudioChunks, { type: studioAudioMime || "audio/webm" });
            if (studioAudioPlaybackEl) {
              studioAudioPlaybackEl.src = URL.createObjectURL(studioAudioBlob);
              studioAudioPlaybackEl.style.display = "block";
            }

            if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Processing…";
            (async () => {
              try {
                const ts = new Date().toISOString().replace(/[:.]/g, "-");
                const ext = (studioAudioBlob.type || "").includes("ogg") ? "ogg" : "webm";
                const fileName = `kat_audio_${ts}.${ext}`;
                studioAudioFilePath = await ensureStudioTempFilePath(studioAudioBlob, fileName);
                if (studioAudioMetaEl) {
                  const kb = Math.round(studioAudioBlob.size / 1024);
                  studioAudioMetaEl.textContent = `Recorded: ${kb} KB (${studioAudioBlob.type || "audio"})`;
                  studioAudioMetaEl.style.display = "block";
                }
                if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Ready";
              } catch (_) {
                if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Ready";
              }
            })();

            if (studioAudioStream) {
              studioAudioStream.getTracks().forEach((t) => t.stop());
              studioAudioStream = null;
            }
          };
          studioAudioRecorder.start();
          if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Recording…";
        }

        function stopStudioAudio() {
          if (studioAudioRecorder && studioAudioRecorder.state !== "inactive") {
            studioAudioRecorder.stop();
          }
        }

        let studioVideoRecorder = null;
        let studioVideoChunks = [];
        let studioVideoBlob = null;
        let studioVideoMime = "";
        let studioVideoStream = null;
        let studioVideoFilePath = "";

        function clearStudioVideoTake() {
          studioVideoBlob = null;
          studioVideoChunks = [];
          studioVideoFilePath = "";
          if (studioVideoStream) {
            try {
              studioVideoStream.getTracks().forEach((t) => t.stop());
            } catch (_) {
              // ignore
            }
            studioVideoStream = null;
          }
          if (studioVideoPlaybackEl) {
            try {
              if (studioVideoPlaybackEl.src) URL.revokeObjectURL(studioVideoPlaybackEl.src);
            } catch (_) {
              // ignore
            }
            studioVideoPlaybackEl.src = "";
            studioVideoPlaybackEl.style.display = "none";
          }
          if (studioVideoLiveEl) {
            studioVideoLiveEl.srcObject = null;
            studioVideoLiveEl.style.display = "none";
          }
          if (studioVideoMetaEl) {
            studioVideoMetaEl.style.display = "none";
            studioVideoMetaEl.textContent = "";
          }
          if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Idle";
        }

        async function startStudioVideo() {
          if (!navigator.mediaDevices?.getUserMedia || !navigator.mediaDevices?.getDisplayMedia) {
            throw new Error("Video recording not supported");
          }
          clearStudioVideoTake();

          const source = studioVideoSourceEl?.value || "camera";
          const camId = studioVideoCamEl?.value || "";
          const micId = studioVideoMicEl?.value || "";

          let audioStream = null;
          let videoStream = null;
          try {
            audioStream = await navigator.mediaDevices.getUserMedia({
              audio: micId ? { deviceId: { exact: micId } } : true,
              video: false,
            });

            if (source === "screen") {
              videoStream = await navigator.mediaDevices.getDisplayMedia({ video: true, audio: false });
            } else {
              videoStream = await navigator.mediaDevices.getUserMedia({
                video: camId ? { deviceId: { exact: camId } } : true,
                audio: false,
              });
            }
          } catch (e) {
            try {
              if (videoStream) videoStream.getTracks().forEach((t) => t.stop());
            } catch (_) {
              // ignore
            }
            try {
              if (audioStream) audioStream.getTracks().forEach((t) => t.stop());
            } catch (_) {
              // ignore
            }
            throw e;
          }

          const tracks = [...videoStream.getVideoTracks(), ...audioStream.getAudioTracks()];
          studioVideoStream = new MediaStream(tracks);

          try {
            const vTrack = videoStream.getVideoTracks()[0];
            if (vTrack) {
              vTrack.addEventListener("ended", () => {
                try {
                  stopStudioVideo();
                } catch (_) {}
              });
            }
          } catch (_) {}

          if (studioVideoLiveEl) {
            studioVideoLiveEl.srcObject = studioVideoStream;
            studioVideoLiveEl.style.display = "block";
            try {
              await studioVideoLiveEl.play();
            } catch (_) {}
          }

          studioVideoMime = pickFirstSupportedMime([
            "video/webm;codecs=vp9,opus",
            "video/webm;codecs=vp8,opus",
            "video/webm;codecs=vp9",
            "video/webm;codecs=vp8",
            "video/webm",
          ]);
          const opts = studioVideoMime ? { mimeType: studioVideoMime } : undefined;
          studioVideoChunks = [];
          studioVideoRecorder = new MediaRecorder(studioVideoStream, opts);
          studioVideoRecorder.ondataavailable = (e) => {
            if (e.data && e.data.size > 0) studioVideoChunks.push(e.data);
          };
          studioVideoRecorder.onstop = () => {
            studioVideoBlob = new Blob(studioVideoChunks, { type: studioVideoMime || "video/webm" });
            if (studioVideoPlaybackEl) {
              studioVideoPlaybackEl.src = URL.createObjectURL(studioVideoBlob);
              studioVideoPlaybackEl.style.display = "block";
            }
            if (studioVideoLiveEl) {
              studioVideoLiveEl.srcObject = null;
              studioVideoLiveEl.style.display = "none";
            }

            if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Processing…";
            (async () => {
              try {
                const ts = new Date().toISOString().replace(/[:.]/g, "-");
                const fileName = `kat_video_${ts}.webm`;
                studioVideoFilePath = await ensureStudioTempFilePath(studioVideoBlob, fileName);
                if (studioVideoMetaEl) {
                  const mb = (studioVideoBlob.size / (1024 * 1024)).toFixed(2);
                  studioVideoMetaEl.textContent = `Recorded: ${mb} MB (${studioVideoBlob.type || "video"})`;
                  studioVideoMetaEl.style.display = "block";
                }
                if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Ready";
              } catch (_) {
                if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Ready";
              }
            })();

            if (studioVideoStream) {
              studioVideoStream.getTracks().forEach((t) => t.stop());
              studioVideoStream = null;
            }
          };
          studioVideoRecorder.start();
          if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Recording…";
        }

        function stopStudioVideo() {
          if (studioVideoRecorder && studioVideoRecorder.state !== "inactive") {
            studioVideoRecorder.stop();
            return;
          }

          // If recorder is already inactive (or failed), ensure streams are stopped.
          if (studioVideoStream) {
            try {
              studioVideoStream.getTracks().forEach((t) => t.stop());
            } catch (_) {
              // ignore
            }
            studioVideoStream = null;
          }
          if (studioVideoLiveEl) {
            studioVideoLiveEl.srcObject = null;
            studioVideoLiveEl.style.display = "none";
          }

          if (studioVideoStatusEl && studioVideoStatusEl.textContent === "Recording…") {
            studioVideoStatusEl.textContent = "Idle";
          }
        }

        if (studioAudioStartEl) {
          studioAudioStartEl.addEventListener("click", async () => {
            setError("");
            try {
              if (studioAudioRecorder && studioAudioRecorder.state !== "inactive") {
                showModal({
                  title: "Already recording",
                  body: "Audio recording is already in progress.",
                  actions: [{ label: "OK", primary: true }],
                });
                return;
              }

              showModal({
                title: "Ready to record audio?",
                body: "Click Start to begin recording. You can stop anytime.",
                actions: [
                  {
                    label: "Start",
                    primary: true,
                    onClick: async () => {
                      await refreshStudioDevices();
                      await startStudioAudio();
                    },
                  },
                  { label: "Cancel" },
                ],
              });
            } catch (e) {
              setError(String(e));
              showModal({ title: "Audio recording error", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioAudioStopEl) {
          studioAudioStopEl.addEventListener("click", async () => {
            setError("");
            try {
              stopStudioAudio();
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (studioAudioDeleteEl) {
          studioAudioDeleteEl.addEventListener("click", async () => {
            showModal({
              title: "Delete recording?",
              body: "This will discard the current audio take.",
              actions: [
                {
                  label: "Delete",
                  primary: true,
                  onClick: async () => {
                    clearStudioAudioTake();
                  },
                },
                { label: "Cancel" },
              ],
            });
          });
        }

        if (studioAudioSaveEl) {
          studioAudioSaveEl.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioAudioBlob) {
                showModal({ title: "Nothing to save", body: "Record audio first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const ext = (studioAudioBlob.type || "").includes("ogg") ? "ogg" : "webm";
              const ts = new Date().toISOString().replace(/[:.]/g, "-");
              const out = await saveBlobToFile(studioAudioBlob, `kat_audio_${ts}.${ext}`, [{ name: ext.toUpperCase(), extensions: [ext] }]);
              if (out) {
                showModal({ title: "Saved", body: `Saved to:\n${out}`, actions: [{ label: "OK", primary: true }] });
              }
            } catch (e) {
              setError(String(e));
              showModal({ title: "Save failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioAudioSendEl) {
          studioAudioSendEl.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioAudioBlob) {
                showModal({ title: "Nothing to send", body: "Record audio first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const toAddressInput = studioAudioToEl?.value.trim() || "";
              const knsNetwork = knsNetworkFromRpcUrl(studioAudioRpcEl?.value || "");
              const cached = getCachedResolved(studioAudioToResolvedEl, toAddressInput, knsNetwork);
              const toAddress = cached || (await resolveToAddressMaybeKns(toAddressInput, knsNetwork));
              if (studioAudioToEl) studioAudioToEl.value = toAddress;
              const amountKas = Number(studioAudioAmountEl?.value || "0");
              const rpcUrl = studioAudioRpcEl?.value.trim() || "";
              const fromPrivateKey = isWalletSessionUnlocked ? "" : (studioAudioPrivEl?.value.trim() || "");

              if (!toAddress) {
                setError("Enter to address.");
                return;
              }
              if (!Number.isFinite(amountKas) || amountKas <= 0) {
                setError("Enter an amount > 0.");
                return;
              }
              if (!isWalletSessionUnlocked && !fromPrivateKey) {
                setError("Enter a private key or unlock the wallet to sign automatically.");
                return;
              }

              const kb = Math.round(studioAudioBlob.size / 1024);
              showModal({
                title: "Send recording?",
                body: `Audio size: ${kb} KB\n\nYou can playback above before sending.`,
                actions: [
                  {
                    label: "Send",
                    primary: true,
                    onClick: async () => {
                      let overlayWasShown = false;
                      try {
                        showActionOverlay({ title: "Publishing…", sub: "Submitting recording to the DAG", theme: "project" });
                        overlayWasShown = true;
                        if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Sending…";
                        const ts = new Date().toISOString().replace(/[:.]/g, "-");
                        const ext = (studioAudioBlob.type || "").includes("ogg") ? "ogg" : "webm";
                        const fileName = `kat_audio_${ts}.${ext}`;
                        if (!studioAudioFilePath) {
                          studioAudioFilePath = await ensureStudioTempFilePath(studioAudioBlob, fileName);
                        }
                        const txid = await tauri.invoke("wallet_send_file_path", {
                          accountId: null,
                          filePath: studioAudioFilePath,
                          toAddress,
                          amountKas,
                          rpcUrl,
                          resumeFrom: null,
                          resumeOutputIndex: 1,
                          fileName,
                          fromPrivateKey,
                        });
                        try {
                          sendTxidEl.textContent = String(txid);
                          recvTxEl.value = String(txid);
                          sendProgressEl.textContent = "Done";
                          setRing(sendRingEl, 1);
                        } catch (_) {}
                        if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Sent";
                        if (overlayWasShown) {
                          hideActionOverlay();
                          overlayWasShown = false;
                        }
                        showModal({
                          title: "Send complete",
                          body: `Transaction ID:\n${String(txid)}`,
                          actions: [
                            { label: "OK", primary: true },
                            {
                              label: "Open in Explorer",
                              onClick: async () => openExternal(getExplorerTxUrl(String(txid))),
                            },
                          ],
                        });
                      } finally {
                        if (overlayWasShown) hideActionOverlay();
                      }
                    },
                  },
                  { label: "Cancel" },
                ],
              });
            } catch (e) {
              if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Failed";
              setError(String(e));
              showModal({ title: "Send failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioVideoStartEl) {
          studioVideoStartEl.addEventListener("click", async () => {
            setError("");
            try {
              if (studioVideoRecorder && studioVideoRecorder.state !== "inactive") {
                showModal({
                  title: "Already recording",
                  body: "Video recording is already in progress.",
                  actions: [{ label: "OK", primary: true }],
                });
                return;
              }

              const source = studioVideoSourceEl?.value || "camera";
              const sourceLabel = source === "screen" ? "Screen" : "Camera";
              showModal({
                title: "Ready to record video?",
                body: `Source: ${sourceLabel}\n\nClick Start to begin recording. You can stop anytime.`,
                actions: [
                  {
                    label: "Start",
                    primary: true,
                    onClick: async () => {
                      await refreshStudioDevices();
                      await startStudioVideo();
                    },
                  },
                  { label: "Cancel" },
                ],
              });
            } catch (e) {
              setError(String(e));
              showModal({ title: "Video recording error", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioVideoStopEl) {
          studioVideoStopEl.addEventListener("click", async () => {
            setError("");
            try {
              stopStudioVideo();
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (studioVideoDeleteEl) {
          studioVideoDeleteEl.addEventListener("click", async () => {
            showModal({
              title: "Delete recording?",
              body: "This will discard the current video take.",
              actions: [
                {
                  label: "Delete",
                  primary: true,
                  onClick: async () => {
                    clearStudioVideoTake();
                  },
                },
                { label: "Cancel" },
              ],
            });
          });
        }

        if (studioVideoSaveEl) {
          studioVideoSaveEl.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioVideoBlob) {
                showModal({ title: "Nothing to save", body: "Record video first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const ts = new Date().toISOString().replace(/[:.]/g, "-");
              const out = await saveBlobToFile(studioVideoBlob, `kat_video_${ts}.webm`, [{ name: "WEBM", extensions: ["webm"] }]);
              if (out) {
                showModal({ title: "Saved", body: `Saved to:\n${out}`, actions: [{ label: "OK", primary: true }] });
              }
            } catch (e) {
              setError(String(e));
              showModal({ title: "Save failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioVideoSendEl) {
          studioVideoSendEl.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioVideoBlob) {
                showModal({ title: "Nothing to send", body: "Record video first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const toAddressInput = studioVideoToEl?.value.trim() || "";
              const knsNetwork = knsNetworkFromRpcUrl(studioVideoRpcEl?.value || "");
              const cached = getCachedResolved(studioVideoToResolvedEl, toAddressInput, knsNetwork);
              const toAddress = cached || (await resolveToAddressMaybeKns(toAddressInput, knsNetwork));
              if (studioVideoToEl) studioVideoToEl.value = toAddress;
              const amountKas = Number(studioVideoAmountEl?.value || "0");
              const rpcUrl = studioVideoRpcEl?.value.trim() || "";
              const fromPrivateKey = isWalletSessionUnlocked ? "" : (studioVideoPrivEl?.value.trim() || "");

              if (!toAddress) {
                setError("Enter to address.");
                return;
              }
              if (!Number.isFinite(amountKas) || amountKas <= 0) {
                setError("Enter an amount > 0.");
                return;
              }
              if (!isWalletSessionUnlocked && !fromPrivateKey) {
                setError("Enter a private key or unlock the wallet to sign automatically.");
                return;
              }

              const mb = (studioVideoBlob.size / (1024 * 1024)).toFixed(2);
              showModal({
                title: "Send recording?",
                body: `Video size: ${mb} MB\n\nYou can playback above before sending.`,
                actions: [
                  {
                    label: "Send",
                    primary: true,
                    onClick: async () => {
                      let overlayWasShown = false;
                      try {
                        showActionOverlay({ title: "Publishing…", sub: "Submitting recording to the DAG", theme: "project" });
                        overlayWasShown = true;
                        if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Sending…";
                        const ts = new Date().toISOString().replace(/[:.]/g, "-");
                        const fileName = `kat_video_${ts}.webm`;
                        if (!studioVideoFilePath) {
                          studioVideoFilePath = await ensureStudioTempFilePath(studioVideoBlob, fileName);
                        }
                        const txid = await tauri.invoke("wallet_send_file_path", {
                          accountId: null,
                          filePath: studioVideoFilePath,
                          toAddress,
                          amountKas,
                          rpcUrl,
                          resumeFrom: null,
                          resumeOutputIndex: 1,
                          fileName,
                          fromPrivateKey,
                        });
                        try {
                          sendTxidEl.textContent = String(txid);
                          recvTxEl.value = String(txid);
                          sendProgressEl.textContent = "Done";
                          setRing(sendRingEl, 1);
                        } catch (_) {}
                        if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Sent";
                        if (overlayWasShown) {
                          hideActionOverlay();
                          overlayWasShown = false;
                        }
                        showModal({
                          title: "Send complete",
                          body: `Transaction ID:\n${String(txid)}`,
                          actions: [
                            { label: "OK", primary: true },
                            {
                              label: "Open in Explorer",
                              onClick: async () => openExternal(getExplorerTxUrl(String(txid))),
                            },
                          ],
                        });
                      } finally {
                        if (overlayWasShown) hideActionOverlay();
                      }
                    },
                  },
                  { label: "Cancel" },
                ],
              });
            } catch (e) {
              if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Failed";
              setError(String(e));
              showModal({ title: "Send failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        if (studioVideoExportMp4El) {
          studioVideoExportMp4El.addEventListener("click", async () => {
            setError("");
            try {
              if (!studioVideoBlob) {
                showModal({ title: "Nothing to export", body: "Record video first.", actions: [{ label: "OK", primary: true }] });
                return;
              }
              const ts = new Date().toISOString().replace(/[:.]/g, "-");
              const inName = `kat_video_${ts}.webm`;
              if (!studioVideoFilePath) {
                studioVideoFilePath = await ensureStudioTempFilePath(studioVideoBlob, inName);
              }
              const outPath = await dialogApi.save({
                defaultPath: `kat_video_${ts}.mp4`,
                filters: [{ name: "MP4", extensions: ["mp4"] }],
              });
              if (!outPath) return;
              if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Exporting…";
              await tauri.invoke("ffmpeg_transcode", {
                inputPath: studioVideoFilePath,
                outputPath: String(outPath),
                kind: "mp4",
              });
              if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Ready";
              showModal({ title: "Export complete", body: `Saved to:\n${String(outPath)}`, actions: [{ label: "OK", primary: true }] });
            } catch (e) {
              if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Ready";
              setError(String(e));
              showModal({ title: "Export failed", body: String(e), actions: [{ label: "Close", primary: true }] });
            }
          });
        }

        function buildFullPath() {
          const base = String(walletBasePathEl?.value || "").trim();
          const chain = String(walletChainEl?.value || "").trim();
          const indexRaw = String(walletAddrIndexEl?.value || "").trim();
          const idx = Number(indexRaw);

          if (!base) {
            setError("Enter a base derivation path.");
            return "";
          }
          if (!base.startsWith("m/")) {
            setError("Base path must start with m/ (example: m/44'/111111'/0')");
            return "";
          }
          if (!Number.isFinite(idx) || idx < 0 || !Number.isInteger(idx)) {
            setError("Index must be a non-negative integer.");
            return "";
          }

          const cleanedBase = base.replace(/\/+$/, "");
          const cleanedChain = chain === "0" || chain === "1" ? chain : "0";
          return `${cleanedBase}/${cleanedChain}/${idx}`;
        }

        async function refreshWalletAddressAndBalance() {
          const path = buildFullPath();
          if (!path) return;
          walletFullPathEl.textContent = path;

          const network = walletNetworkEl.value;
          const rpcUrl = walletRpcUrlEl.value.trim();

          const addr = await tauri.invoke("wallet_derive_receive_address", {
            network,
            derivationPath: path,
          });
          walletDerivedAddressEl.textContent = String(addr);

          try {
            walletBalanceEl.textContent = "…";
            syncWalletHeroBalance();
            const bal = await tauri.invoke("wallet_get_balance", {
              network,
              derivationPath: path,
              rpcUrl: rpcUrl || null,
            });
            const b = Number(bal);
            walletBalanceEl.textContent = Number.isFinite(b) ? b.toFixed(8) : "—";
            syncWalletHeroBalance();
          } catch (_) {
            walletBalanceEl.textContent = "—";
            syncWalletHeroBalance();
          }

          try {
            await refreshWalletTxHistory();
          } catch (_) {
            // ignore
          }
        }

        async function refreshProfiles() {
          try {
            const profiles = await tauri.invoke("wallet_profiles_list", {});
            const prev = walletProfileSelectEl.value;
            walletProfileSelectEl.innerHTML = "";
            const opt0 = document.createElement("option");
            opt0.value = "";
            opt0.textContent = "(none)";
            walletProfileSelectEl.appendChild(opt0);
            (profiles || []).forEach((p) => {
              const o = document.createElement("option");
              o.value = p.username;
              o.textContent = p.username;
              walletProfileSelectEl.appendChild(o);
            });
            if (prev) walletProfileSelectEl.value = prev;
          } catch (e) {
            setError(String(e));
          }
        }

        await refreshProfiles();
        await refreshWalletStatus();

        walletRefreshBtnEl.addEventListener("click", async () => {
          setError("");
          await refreshProfiles();
          await refreshWalletStatus();
        });

        walletCreateBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const username = walletNewUsernameEl.value.trim();
            const password = walletNewPasswordEl.value;
            const wordCount = Number(walletWordCountEl.value || "24");
            const phrase = await tauri.invoke("wallet_profile_create_mnemonic", {
              username,
              password,
              wordCount,
            });
            await refreshProfiles();
            showModal({
              title: "Mnemonic created",
              body: `Save this mnemonic securely. Anyone with it can control your funds.\n\n${String(phrase)}`,
              actions: [
                {
                  label: "Copy mnemonic",
                  keepOpen: true,
                  onClick: async () => {
                    await copyText(String(phrase));
                    flashPill("Copied");
                  },
                },
                { label: "I saved it", primary: true },
              ],
            });
          } catch (e) {
            setError(String(e));
          }
        });

        if (walletImportMnemonicBtnEl) {
          walletImportMnemonicBtnEl.addEventListener("click", async () => {
            setError("");
            try {
              const username = String(walletImportUsernameEl?.value || "").trim();
              const password = String(walletImportPasswordEl?.value || "");
              const phrase = String(walletImportMnemonicEl?.value || "").trim();
              const mnemonicPasswordRaw = String(walletMnemonicPassEl?.value || "");
              const mnemonicPassword = mnemonicPasswordRaw.trim() ? mnemonicPasswordRaw : null;

              if (!username) {
                setError("Enter a username.");
                return;
              }
              if (!password) {
                setError("Enter a password.");
                return;
              }
              if (!phrase) {
                setError("Enter your mnemonic phrase.");
                return;
              }

              await tauri.invoke("wallet_profile_import_mnemonic", {
                username,
                password,
                phrase,
                mnemonicPassword,
              });

              await refreshProfiles();
              showModal({
                title: "Imported",
                body: `Mnemonic imported for profile "${username}".`,
                actions: [{ label: "OK", primary: true }],
              });
            } catch (e) {
              setError(String(e));
              showModal({
                title: "Import failed",
                body: String(e || "Unable to import mnemonic."),
                actions: [{ label: "OK", primary: true }],
              });
            }
          });
        }

        if (walletImportPrivKeyBtnEl) {
          walletImportPrivKeyBtnEl.addEventListener("click", async () => {
            setError("");
            try {
              const username = String(walletImportUsernameEl?.value || "").trim();
              const password = String(walletImportPasswordEl?.value || "");
              const privateKeyHex = String(walletImportPrivKeyEl?.value || "").trim();

              if (!username) {
                setError("Enter a username.");
                return;
              }
              if (!password) {
                setError("Enter a password.");
                return;
              }
              if (!privateKeyHex) {
                setError("Enter a private key (hex).");
                return;
              }

              await tauri.invoke("wallet_profile_import_private_key", {
                username,
                password,
                privateKeyHex,
              });

              await refreshProfiles();
              showModal({
                title: "Imported",
                body: `Private key imported for profile "${username}".`,
                actions: [{ label: "OK", primary: true }],
              });
            } catch (e) {
              setError(String(e));
              showModal({
                title: "Import failed",
                body: String(e || "Unable to import private key."),
                actions: [{ label: "OK", primary: true }],
              });
            }
          });
        }

        if (walletDeleteProfileBtnEl) {
          walletDeleteProfileBtnEl.addEventListener("click", async () => {
            setError("");
            const username = walletProfileSelectEl.value.trim();
            if (!username) {
              setError("Select a profile to delete.");
              return;
            }

            showModal({
              title: "Delete profile?",
              body: `This will permanently remove the profile "${username}" from this device.\n\nThis cannot be undone.`,
              actions: [
                {
                  label: "Delete",
                  primary: true,
                  onClick: async () => {
                    await tauri.invoke("wallet_profile_delete", { username });
                    walletPasswordEl.value = "";
                    walletProfileSelectEl.value = "";
                    await refreshProfiles();
                    await refreshWalletStatus();
                  },
                },
                { label: "Cancel" },
              ],
            });
          });
        }

        if (walletClearAllProfilesBtnEl) {
          walletClearAllProfilesBtnEl.addEventListener("click", async () => {
            setError("");
            showModal({
              title: "Clear ALL wallet data?",
              body: "This will remove ALL saved wallet profiles (usernames/accounts) from this device.\n\nThis cannot be undone.",
              actions: [
                {
                  label: "Clear all",
                  primary: true,
                  onClick: async () => {
                    await tauri.invoke("wallet_profiles_clear_all", {});
                    walletPasswordEl.value = "";
                    walletProfileSelectEl.value = "";
                    await refreshProfiles();
                    await refreshWalletStatus();
                  },
                },
                { label: "Cancel" },
              ],
            });
          });
        }

        if (walletCopyPathBtnEl) {
          walletCopyPathBtnEl.addEventListener("click", async () => {
            try {
              await copyText(walletFullPathEl.textContent);
              flashPill("Copied path");
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (walletCopyAddressBtnEl) {
          walletCopyAddressBtnEl.addEventListener("click", async () => {
            try {
              await copyText(walletDerivedAddressEl.textContent);
              flashPill("Copied address");
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (walletCopySendKasTxidBtnEl) {
          walletCopySendKasTxidBtnEl.addEventListener("click", async () => {
            try {
              await copyText(walletSendKasTxidEl.textContent);
              flashPill("Copied TXID");
            } catch (e) {
              setError(String(e));
            }
          });
        }

        walletUnlockBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const username = walletProfileSelectEl.value.trim();
            const password = walletPasswordEl.value;

            await tauri.invoke("wallet_unlock", { username, password });
            await refreshWalletStatus();
            await refreshWalletAddressAndBalance();
            try {
              document.getElementById("walletFsClose")?.click();
            } catch (_) {
              // ignore
            }
            showModal({
              title: "Unlocked",
              body: username ? `Wallet unlocked for profile "${username}".` : "Wallet unlocked.",
              actions: [{ label: "OK", primary: true }],
            });
          } catch (e) {
            setError(String(e));
            try {
              document.getElementById("walletFsClose")?.click();
            } catch (_) {
              // ignore
            }
            const msg = String(e || "");
            const lower = msg.toLowerCase();
            const hint =
              lower.includes("password") || lower.includes("invalid") || lower.includes("decrypt")
                ? "Incorrect password (or invalid profile)."
                : "Unable to unlock wallet.";
            showModal({
              title: "Unlock failed",
              body: msg ? `${hint}\n\n${msg}` : hint,
              actions: [{ label: "OK", primary: true }],
            });
          }
        });

        walletLockBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            await tauri.invoke("wallet_lock", {});
            walletPasswordEl.value = "";
            walletProfileSelectEl.value = "";
            await refreshWalletStatus();
            try {
              document.getElementById("walletFsClose")?.click();
            } catch (_) {
              // ignore
            }
            showModal({
              title: "Locked",
              body: "Wallet successfully locked.",
              actions: [{ label: "OK", primary: true }],
            });
          } catch (e) {
            setError(String(e));
            showModal({
              title: "Lock failed",
              body: String(e || "Unable to lock wallet."),
              actions: [{ label: "OK", primary: true }],
            });
          }
        });

        walletDeriveBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const network = walletNetworkEl.value;
            const path = buildFullPath();
            walletFullPathEl.textContent = path || "—";
            const addr = await tauri.invoke("wallet_derive_receive_address", {
              network,
              derivationPath: path,
            });
            walletDerivedAddressEl.textContent = String(addr);
            sendToEl.value = String(addr);
            await refreshWalletAddressAndBalance();
          } catch (e) {
            setError(String(e));
          }
        });

        walletBalanceRefreshBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            await refreshWalletAddressAndBalance();
          } catch (e) {
            setError(String(e));
          }
        });

        walletSendKasBtnEl.addEventListener("click", async () => {
          setError("");
          let overlayWasShown = false;
          try {
            showActionOverlay({ title: "Sending…", sub: "Broadcasting transaction to the DAG", theme: "kaspa" });
            overlayWasShown = true;
            const network = walletNetworkEl.value;
            const rpcUrl = walletRpcUrlEl.value.trim();
            const derivationPath = buildFullPath();
            const toAddressInput = walletSendToEl.value.trim();
            const knsNetwork = walletNetworkToKnsNetwork(network);
            if (!knsNetwork && !toAddressInput.includes(":")) {
              throw new Error("KNS is not supported on this network");
            }
            const cached = getCachedResolved(walletSendToResolvedEl, toAddressInput, knsNetwork);
            const toAddress = cached || (await resolveToAddressMaybeKns(toAddressInput, knsNetwork));
            walletSendToEl.value = toAddress;
            const amountKas = Number(walletSendAmountEl.value || "0");

            if (!toAddress) {
              setError("Enter a destination address.");
              return;
            }
            if (!Number.isFinite(amountKas) || amountKas <= 0) {
              setError("Enter an amount > 0.");
              return;
            }

            walletSendKasTxidEl.textContent = "Working…";
            const txid = await tauri.invoke("wallet_send_kas", {
              network,
              derivationPath,
              rpcUrl: rpcUrl || null,
              toAddress,
              amountKas,
            });
            walletSendKasTxidEl.textContent = String(txid);
            if (overlayWasShown) {
              hideActionOverlay();
              overlayWasShown = false;
            }
            showModal({
              title: "Send complete",
              body: `Transaction ID:\n${String(txid)}`,
              actions: [
                {
                  label: "Open in Explorer",
                  primary: true,
                  onClick: () => openExternal(getExplorerTxUrl(txid)),
                },
                { label: "Close" },
              ],
            });
            await refreshWalletAddressAndBalance();
          } catch (e) {
            walletSendKasTxidEl.textContent = "—";
            setError(String(e));
            if (overlayWasShown) {
              hideActionOverlay();
              overlayWasShown = false;
            }
          } finally {
            if (overlayWasShown) hideActionOverlay();
          }
        });

        walletDebugBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const out = await tauri.invoke("wallet_debug_unlocked_material_fingerprint", {});
            walletDebugOutEl.textContent = String(out);
          } catch (e) {
            walletDebugOutEl.textContent = "—";
            setError(String(e));
          }
        });

        function setPickedFile(file) {
          selectedSendFile = file || null;
          selectedSendFilePath = "";

          if (!sendFileHintEl) return;
          if (selectedSendFile && selectedSendFile.name) {
            sendFileHintEl.textContent = `Selected: ${selectedSendFile.name}`;
          } else {
            sendFileHintEl.textContent = "Tip: drag & drop a file here.";
          }
        }

        function setPickedFilePath(path) {
          selectedSendFilePath = String(path || "").trim();
          selectedSendFile = null;
          try {
            if (sendFileEl) sendFileEl.value = "";
          } catch (_) {}

          if (!sendFileHintEl) return;
          if (selectedSendFilePath) {
            const base = selectedSendFilePath.split(/[/\\]/).pop() || selectedSendFilePath;
            sendFileHintEl.textContent = `Selected: ${base}`;
          } else {
            sendFileHintEl.textContent = "Tip: drag & drop a file here.";
          }
        }

        function setDroppedFile(file) {
          setPickedFile(file);
        }

        if (sendDropZoneEl && dialogApi?.open) {
          try {
            if (sendFileEl) {
              sendFileEl.style.pointerEvents = "none";
            }
          } catch (_) {}

          sendDropZoneEl.style.cursor = "pointer";

          const pickPath = async () => {
            const picked = await dialogApi.open({ directory: false, multiple: false });
            if (!picked) return;
            setPickedFilePath(String(picked));
          };

          sendDropZoneEl.addEventListener("click", (e) => {
            try {
              e.preventDefault();
              e.stopPropagation();
            } catch (_) {}
            pickPath();
          });
          sendDropZoneEl.addEventListener("keydown", (e) => {
            const k = e && (e.key || e.code);
            if (k !== "Enter" && k !== " ") return;
            try {
              e.preventDefault();
              e.stopPropagation();
            } catch (_) {}
            pickPath();
          });
        }

        function clearSession() {
          setError("");

          try {
            closeModal();
          } catch (_) {}

          try {
            setPickedFile(null);
          } catch (_) {}
          try {
            if (sendFileEl) sendFileEl.value = "";
          } catch (_) {}

          try {
            if (sendProgressEl) sendProgressEl.textContent = "Idle";
            if (sendTxidEl) sendTxidEl.textContent = "—";
            setRing(sendRingEl, 0);
          } catch (_) {}

          try {
            if (sendToEl) sendToEl.value = "";
            if (sendAmountEl) sendAmountEl.value = "0.15";
            clearKnsPreview(sendToResolvedEl);
          } catch (_) {}

          try {
            if (recvStatusEl) recvStatusEl.textContent = "Idle";
            setRing(recvRingEl, 0);
            if (recvTxEl) recvTxEl.value = "";
            if (recvStartEl) recvStartEl.value = "";
            if (recvOutEl) recvOutEl.value = "";
            lastReceivedPath = "";
          } catch (_) {}

          try {
            stopStudioAudio();
          } catch (_) {}
          try {
            stopStudioVideo();
          } catch (_) {}
          try {
            clearStudioAudioTake();
          } catch (_) {}
          try {
            clearStudioVideoTake();
          } catch (_) {}

          try {
            if (studioAudioToEl) studioAudioToEl.value = "";
            if (studioAudioAmountEl) studioAudioAmountEl.value = "0.15";
            clearKnsPreview(studioAudioToResolvedEl);
          } catch (_) {}
          try {
            if (studioVideoToEl) studioVideoToEl.value = "";
            if (studioVideoAmountEl) studioVideoAmountEl.value = "0.15";
            clearKnsPreview(studioVideoToResolvedEl);
          } catch (_) {}
        }

        sendBtn.addEventListener("click", async () => {
          setError("");
          sendTxidEl.textContent = "—";
          sendProgressEl.textContent = "Working…";
          setRing(sendRingEl, 0);

          const f = selectedSendFile || (sendFileEl.files && sendFileEl.files[0]);
          if (!f && !selectedSendFilePath) {
            sendProgressEl.textContent = "Idle";
            setError("Select a file to send.");
            return;
          }

          const rpcUrl = sendRpcEl.value.trim();
          const knsNetwork = knsNetworkFromRpcUrl(rpcUrl);
          const fromPrivateKey = sendPrivEl.value.trim();
          const toAddressInput = sendToEl.value.trim();
          const cached = getCachedResolved(sendToResolvedEl, toAddressInput, knsNetwork);
          const toAddress = cached || (await resolveToAddressMaybeKns(toAddressInput, knsNetwork));
          sendToEl.value = toAddress;
          const amountKas = Number(sendAmountEl.value || "0");

          if (!fromPrivateKey) {
            try {
              const unlocked = await tauri.invoke("wallet_unlocked_username", {});
              if (!unlocked) {
                sendProgressEl.textContent = "Idle";
                setError("Unlock your wallet (Wallet → Unlock) or enter a from private key.");
                return;
              }
            } catch (_) {
              sendProgressEl.textContent = "Idle";
              setError("Unlock your wallet (Wallet → Unlock) or enter a from private key.");
              return;
            }
          }
          if (!toAddress) {
            sendProgressEl.textContent = "Idle";
            setError("Enter to address.");
            return;
          }

          let overlayWasShown = false;
          try {
            let txid = "";

            const maxInlineB64Bytes = 8 * 1024 * 1024;
            let filePath = selectedSendFilePath;
            const fileName = (f && f.name) ? f.name : (filePath.split(/[/\\]/).pop() || "upload.bin");

            if (!filePath && f && typeof f.path === "string" && f.path.trim()) {
              filePath = f.path.trim();
            }

            if (!filePath && dialogApi?.open && f && f.size > maxInlineB64Bytes) {
              const picked = await dialogApi.open({ directory: false, multiple: false });
              if (!picked) throw new Error("File selection cancelled");
              filePath = String(picked);
              setPickedFilePath(filePath);
            }

            showActionOverlay({ title: "Publishing…", sub: "Building and submitting chunks to the DAG", theme: "project" });
            overlayWasShown = true;

            if (filePath) {
              txid = await tauri.invoke("wallet_send_file_path", {
                accountId: null,
                filePath,
                toAddress,
                amountKas,
                rpcUrl,
                resumeFrom: null,
                resumeOutputIndex: 1,
                fileName,
                fromPrivateKey,
              });
            } else {
              const fileB64 = await readFileB64(f);
              if (!fileB64) {
                throw new Error("Unable to read file into memory (fileB64 is empty). For large files, use the native file picker.");
              }
              txid = await tauri.invoke("wallet_send_file_b64", {
                accountId: null,
                fileB64,
                toAddress,
                amountKas,
                rpcUrl,
                resumeFrom: null,
                resumeOutputIndex: 1,
                fileName,
                fromPrivateKey,
              });
            }
            sendTxidEl.textContent = String(txid);
            recvTxEl.value = String(txid);
            sendProgressEl.textContent = "Done";
            setRing(sendRingEl, 1);

            if (overlayWasShown) {
              hideActionOverlay();
              overlayWasShown = false;
            }
            showModal({
              title: "Send complete",
              body: `Transaction ID:\n${txid}\n\nNext: open the explorer transaction and copy the first value under \"Block hashes\" to use as a scan anchor for receive.`,
              actions: [
                {
                  label: "Open in Explorer",
                  primary: true,
                  onClick: () => openExternal(getExplorerTxUrl(txid)),
                },
                { label: "Close" },
              ],
            });
          } catch (e) {
            sendProgressEl.textContent = "Failed";
            setRing(sendRingEl, 0);
            setError(String(e));

            showModal({
              title: "Send failed",
              body: String(e),
              actions: [{ label: "Close", primary: true }],
            });
          } finally {
            if (overlayWasShown) hideActionOverlay();
          }
        });

        sendExplorerLinkEl.addEventListener("click", async (e) => {
          e.preventDefault();
          await openExternal(getExplorerTxUrl(sendTxidEl.textContent));
        });

        if (sendCopyTxidBtnEl) {
          sendCopyTxidBtnEl.addEventListener("click", async () => {
            try {
              await copyText(sendTxidEl.textContent);
              flashPill("Copied TXID");
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (recvCopyTxBtnEl) {
          recvCopyTxBtnEl.addEventListener("click", async () => {
            try {
              await copyText(recvTxEl.value);
              flashPill("Copied TXID");
            } catch (e) {
              setError(String(e));
            }
          });
        }

        if (recvCopyStartBtnEl) {
          recvCopyStartBtnEl.addEventListener("click", async () => {
            try {
              await copyText(recvStartEl.value);
              flashPill("Copied block hash");
            } catch (e) {
              setError(String(e));
            }
          });
        }

        recvBtn.addEventListener("click", async () => {
          setError("");
          recvStatusEl.textContent = "Working…";
          setRing(recvRingEl, 0);

          const txId = recvTxEl.value.trim();
          const outputPath = recvOutEl.value.trim();
          const rpcUrl = recvRpcEl.value.trim();
          const startBlockHash = recvStartEl.value.trim();

          if (!txId) {
            recvStatusEl.textContent = "Idle";
            setError("Enter a transaction id.");
            return;
          }
          if (!outputPath) {
            recvStatusEl.textContent = "Idle";
            setError("Enter an output filename/path.");
            return;
          }

          let overlayWasShown = false;
          try {
            showActionOverlay({ title: "Receiving…", sub: "Scanning and downloading chunks from the DAG", theme: "project" });
            overlayWasShown = true;
            const outPath = await tauri.invoke("wallet_receive_file", {
              txId,
              outputPath,
              rpcUrl,
              startBlockHash: startBlockHash || null,
            });
            lastReceivedPath = String(outPath);
            recvStatusEl.textContent = "Done";
            setRing(recvRingEl, 1);

            if (overlayWasShown) {
              hideActionOverlay();
              overlayWasShown = false;
            }

            showModal({
              title: "Receive complete",
              body: `File saved to:\n${lastReceivedPath}`,
              actions: [
                {
                  label: "Open file",
                  primary: true,
                  onClick: () => tauri.invoke("open_file", { path: lastReceivedPath }),
                },
                {
                  label: "Reveal in folder",
                  onClick: () => tauri.invoke("reveal_in_folder", { path: lastReceivedPath }),
                },
                { label: "Close" },
              ],
            });
          } catch (e) {
            recvStatusEl.textContent = "Failed";
            setRing(recvRingEl, 0);
            setError(String(e));

            showModal({
              title: "Receive failed",
              body: String(e),
              actions: [{ label: "Close", primary: true }],
            });
            if (overlayWasShown) {
              hideActionOverlay();
              overlayWasShown = false;
            }
          }
          finally {
            if (overlayWasShown) hideActionOverlay();
          }
        });

        explorerTipEl.addEventListener("click", async (e) => {
          e.preventDefault();
          await openExternal(getExplorerTxUrl(recvTxEl.value));
        });

        openBtn.addEventListener("click", async () => {
          if (!lastReceivedPath) {
            setError("No received file yet.");
            return;
          }
          try {
            await tauri.invoke("open_file", { path: lastReceivedPath });
          } catch (e) {
            setError(String(e));
          }
        });

        revealBtn.addEventListener("click", async () => {
          if (!lastReceivedPath) {
            setError("No received file yet.");
            return;
          }
          try {
            await tauri.invoke("reveal_in_folder", { path: lastReceivedPath });
          } catch (e) {
            setError(String(e));
          }
        });
      }

      function startSplash() {
        return new Promise((resolve) => {
          const splashEl = document.getElementById("splash");
          const splashCanvasEl = document.getElementById("splashCanvas");
          const splashNodesEl = document.getElementById("splashNodes");
          if (!splashEl || !splashCanvasEl || !splashNodesEl) {
            resolve();
            return;
          }

          splashEl.classList.remove("splashFade");
          splashEl.setAttribute("aria-hidden", "false");

          const ctx = splashCanvasEl.getContext("2d");
          if (!ctx) {
            setTimeout(() => {
              splashEl.classList.add("splashFade");
              splashEl.setAttribute("aria-hidden", "true");
              resolve();
            }, 6000);
            return;
          }

          const dpr = Math.max(1, window.devicePixelRatio || 1);
          const nodes = [];
          const edges = [];
          let tips = [];
          let w = 0;
          let h = 0;
          let raf = 0;

          const maxNodes = 26;
          const spawnEveryMs = 240;
          const edgeDrawMs = 520;
          const startAt = (typeof performance !== "undefined" && performance.now) ? performance.now() : Date.now();
          let nextSpawnAt = 0;

          const clamp01 = (x) => Math.max(0, Math.min(1, x));
          const ease = (x) => (x < 0.5 ? 2 * x * x : 1 - Math.pow(-2 * x + 2, 2) / 2);
          const pick = (arr, exclude) => {
            const a = (arr || []).filter((v) => v !== exclude);
            if (a.length === 0) return null;
            return a[Math.floor(Math.random() * a.length)];
          };

          const resize = () => {
            const rect = splashEl.getBoundingClientRect();
            w = Math.max(1, Math.floor(rect.width));
            h = Math.max(1, Math.floor(rect.height));
            splashCanvasEl.width = Math.floor(w * dpr);
            splashCanvasEl.height = Math.floor(h * dpr);
            splashCanvasEl.style.width = `${w}px`;
            splashCanvasEl.style.height = `${h}px`;
            ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
          };

          const cx = () => w * 0.5;

          const nodeSvg = `
            <svg width="46" height="46" viewBox="0 0 22 22" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false">
              <path d="M11 1.6L19.2 6.4V15.6L11 20.4L2.8 15.6V6.4L11 1.6Z" stroke="var(--kaspa-primary)" stroke-width="1.8" stroke-linejoin="round" />
              <path d="M4.9 11H7.2L8.4 8.1L10.1 14.2L11.5 10.3L12.6 12.7H14.9" stroke="var(--kaspa-accent)" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          `;

          const addNode = (x, y) => {
            const el = document.createElement("div");
            el.className = "splashNode";
            el.innerHTML = nodeSvg;
            splashNodesEl.appendChild(el);
            const n = {
              x,
              y,
              el,
              p: Math.random() * Math.PI * 2,
              ax: 4 + Math.random() * 8,
              ay: 3 + Math.random() * 7,
            };
            nodes.push(n);
            return nodes.length - 1;
          };

          const addEdge = (from, to, createdAt) => {
            edges.push({ from, to, createdAt });
          };

          const spawnNode = (now) => {
            const i = nodes.length;
            const prog = clamp01((i - 3) / Math.max(1, (maxNodes - 4)));
            const yBase = h * 0.22 + prog * (h * 0.56);
            const xBase = cx() + (Math.random() - 0.5) * (w * 0.56);
            const idx = addNode(xBase, yBase);

            const p1 = pick(tips, null);
            let p2 = null;
            if (Math.random() < 0.7 && tips.length > 1) p2 = pick(tips, p1);

            [p1, p2].filter((p) => p !== null && p !== undefined).forEach((p) => {
              addEdge(p, idx, now);
              tips = tips.filter((t) => t !== p);
            });
            tips.push(idx);
            if (tips.length > 7) tips = tips.slice(tips.length - 7);
          };

          resize();
          window.addEventListener("resize", resize);

          for (let i = 0; i < 3; i++) {
            const gx = cx() + (i - 1) * 90 + (Math.random() - 0.5) * 20;
            const gy = h * 0.18 + (Math.random() - 0.5) * 16;
            const idx = addNode(gx, gy);
            tips.push(idx);
          }
          nextSpawnAt = spawnEveryMs;

          const tick = (t) => {
            ctx.clearRect(0, 0, w, h);
            const now = t;

            while (now - startAt >= nextSpawnAt && nodes.length < maxNodes) {
              spawnNode(now);
              nextSpawnAt += spawnEveryMs;
            }

            for (let i = 0; i < edges.length; i++) {
              const e = edges[i];
              const a = nodes[e.from];
              const b = nodes[e.to];
              if (!a || !b) continue;

              const p = clamp01((now - e.createdAt) / edgeDrawMs);
              const k = ease(p);

              const ajx = Math.sin(now * 0.00115 + a.p) * a.ax;
              const ajy = Math.cos(now * 0.00105 + a.p) * a.ay;
              const bjx = Math.sin(now * 0.00118 + b.p) * b.ax;
              const bjy = Math.cos(now * 0.00108 + b.p) * b.ay;

              const x1 = a.x + ajx;
              const y1 = a.y + ajy;
              const x2 = b.x + bjx;
              const y2 = b.y + bjy;
              const xe = x1 + (x2 - x1) * k;
              const ye = y1 + (y2 - y1) * k;

              ctx.lineWidth = 1.7;
              ctx.strokeStyle = `rgba(73, 234, 203, ${0.06 + k * 0.18})`;
              ctx.beginPath();
              ctx.moveTo(x1, y1);
              ctx.lineTo(xe, ye);
              ctx.stroke();
            }

            for (let i = 0; i < nodes.length; i++) {
              const n = nodes[i];
              const jx = Math.sin(now * 0.00115 + n.p) * n.ax;
              const jy = Math.cos(now * 0.00105 + n.p) * n.ay;
              const x = n.x + jx;
              const y = n.y + jy;
              n.el.style.left = `${x}px`;
              n.el.style.top = `${y}px`;
              n.el.style.animationDelay = `${Math.round(n.p * 100)}ms`;
            }

            raf = window.requestAnimationFrame(tick);
          };

          raf = window.requestAnimationFrame(tick);

          setTimeout(() => {
            try {
              window.cancelAnimationFrame(raf);
            } catch (_) {}
            try {
              window.removeEventListener("resize", resize);
            } catch (_) {}
            splashEl.classList.add("splashFade");
            splashEl.setAttribute("aria-hidden", "true");
            resolve();
          }, 6000);
        });
      }

      startSplash().then(init).catch(() => init());