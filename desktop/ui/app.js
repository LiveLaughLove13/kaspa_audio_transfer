
      const errorEl = document.getElementById("error");
      const sendBtn = document.getElementById("sendBtn");
      const sendFileEl = document.getElementById("sendFile");
      const sendDropZoneEl = document.getElementById("sendDropZone");
      const sendFileHintEl = document.getElementById("sendFileHint");
      const sendPrivEl = document.getElementById("sendPriv");
      const sendPrivAutoHintEl = document.getElementById("sendPrivAutoHint");
      const sendToEl = document.getElementById("sendTo");
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
      const walletBasePathEl = document.getElementById("walletBasePath");
      const walletChainEl = document.getElementById("walletChain");
      const walletAddrIndexEl = document.getElementById("walletAddrIndex");
      const walletDeriveBtnEl = document.getElementById("walletDeriveBtn");
      const walletFullPathEl = document.getElementById("walletFullPath");
      const walletDerivedAddressEl = document.getElementById("walletDerivedAddress");

      const walletBalanceEl = document.getElementById("walletBalance");
      const walletBalanceRefreshBtnEl = document.getElementById("walletBalanceRefreshBtn");
      const walletSendToEl = document.getElementById("walletSendTo");
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

      let lastReceivedPath = "";
      let selectedSendFile = null;

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

      async function openExternal(url) {
        const shell = window.__TAURI__?.shell;
        if (shell?.open) {
          await shell.open(url);
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

        const tauriClipboard = window.__TAURI__?.clipboard;
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

      function closeModal() {
        modalOverlayEl.style.display = "none";
        modalTitleEl.textContent = "Notification";
        modalBodyEl.textContent = "";
        modalActionsEl.innerHTML = "";
      }

      function clearSession() {
        closeModal();
        setError("");

        sendProgressEl.textContent = "Idle";
        recvStatusEl.textContent = "Idle";
        setRing(sendRingEl, 0);
        setRing(recvRingEl, 0);

        sendTxidEl.textContent = "—";
        lastReceivedPath = "";
        selectedSendFile = null;
        if (sendFileHintEl) sendFileHintEl.textContent = "Tip: drag & drop a file here.";

        if (sendFileEl) sendFileEl.value = "";
        sendPrivEl.value = "";
        sendToEl.value = "";
        sendAmountEl.value = "0";
        sendRpcEl.value = "grpc://127.0.0.1:16110";

        recvTxEl.value = "";
        recvStartEl.value = "";
        recvRpcEl.value = "grpc://127.0.0.1:16110";
        recvOutEl.value = "received.bin";
      }

      function setDroppedFile(file) {
        if (!file) return;
        selectedSendFile = file;
        if (sendFileHintEl) sendFileHintEl.textContent = `Selected: ${file.name}`;
      }

      function setPickedFile(file) {
        selectedSendFile = file || null;
        if (sendFileHintEl) {
          sendFileHintEl.textContent = file ? `Selected: ${file.name}` : "Tip: drag & drop a file here.";
        }
      }

      function showModal({ title, body, actions }) {
        modalTitleEl.textContent = title || "Notification";
        modalBodyEl.textContent = body || "";
        modalActionsEl.innerHTML = "";
        (actions || []).forEach((a) => {
          const b = document.createElement("button");
          b.type = "button";
          b.className = `modalBtn ${a.primary ? "modalBtnPrimary" : ""}`;
          b.textContent = a.label || "Action";
          b.addEventListener("click", async () => {
            try {
              if (a.onClick) await a.onClick();
            } finally {
              if (!a.keepOpen) closeModal();
            }
          });
          modalActionsEl.appendChild(b);
        });
        modalOverlayEl.style.display = "flex";
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

      async function readFileB64(file) {
        return new Promise((resolve, reject) => {
          const r = new FileReader();
          r.onerror = () => reject(new Error("failed reading file"));
          r.onload = () => resolve(String(r.result || ""));
          r.readAsDataURL(file);
        });
      }

      async function init() {
        const tauri = window.__TAURI__?.tauri;
        const eventApi = window.__TAURI__?.event;
        const dialogApi = window.__TAURI__?.dialog;
        const fsApi = window.__TAURI__?.fs;

        let isWalletSessionUnlocked = false;

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

        if (!tauri?.invoke) {
          setError("Tauri API not available. Are you running the desktop app through Tauri?");
          return;
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

        modalCloseEl.addEventListener("click", closeModal);
        modalOverlayEl.addEventListener("click", (e) => {
          if (e.target === modalOverlayEl) closeModal();
        });
        window.addEventListener("keydown", (e) => {
          if (e.key === "Escape") closeModal();
        });

        clearSessionBtnEl.addEventListener("click", () => {
          clearSession();
        });

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
        }

        if (walletTabOverviewEl) walletTabOverviewEl.addEventListener("click", () => setWalletTab("overview"));
        if (walletTabManageEl) walletTabManageEl.addEventListener("click", () => setWalletTab("manage"));
        if (walletTabAdvancedEl) walletTabAdvancedEl.addEventListener("click", () => setWalletTab("advanced"));

        setWalletTab("overview");

        function setActivePage(name) {
          const isWallet = name === "wallet";
          const isTransfer = name === "transfer";
          const isStudio = name === "studio";
          if (pageWalletEl) pageWalletEl.className = `page ${isWallet ? "pageActive" : ""}`.trim();
          if (pageTransferEl) pageTransferEl.className = `page ${isTransfer ? "pageActive" : ""}`.trim();
          if (pageStudioEl) pageStudioEl.className = `page ${isStudio ? "pageActive" : ""}`.trim();
          if (navWalletEl) navWalletEl.className = `navBtn ${isWallet ? "navBtnActive" : ""}`.trim();
          if (navTransferEl) navTransferEl.className = `navBtn ${isTransfer ? "navBtnActive" : ""}`.trim();
          if (navStudioEl) navStudioEl.className = `navBtn ${isStudio ? "navBtnActive" : ""}`.trim();
          if (pageTitleEl) pageTitleEl.textContent = isWallet ? "Wallet" : isStudio ? "Studio" : "Transfer";
        }

        function setActiveTransferTab(name) {
          const isSend = name === "send";
          if (tabSendEl) tabSendEl.className = `tabBtn ${isSend ? "tabBtnActive" : ""}`.trim();
          if (tabReceiveEl) tabReceiveEl.className = `tabBtn ${!isSend ? "tabBtnActive" : ""}`.trim();
          if (transferSendPanelEl) transferSendPanelEl.style.display = isSend ? "block" : "none";
          if (transferReceivePanelEl) transferReceivePanelEl.style.display = isSend ? "none" : "block";
        }

        if (navWalletEl) navWalletEl.addEventListener("click", () => setActivePage("wallet"));
        if (navTransferEl) navTransferEl.addEventListener("click", () => setActivePage("transfer"));
        if (navStudioEl) navStudioEl.addEventListener("click", () => setActivePage("studio"));
        if (tabSendEl) tabSendEl.addEventListener("click", () => setActiveTransferTab("send"));
        if (tabReceiveEl) tabReceiveEl.addEventListener("click", () => setActiveTransferTab("receive"));

        setActivePage("transfer");
        setActiveTransferTab("send");

        await refreshNodeBundleDir();

        function setStudioTab(name) {
          const isAudio = name === "audio";
          if (studioTabAudioEl) studioTabAudioEl.className = `tabBtn ${isAudio ? "tabBtnActive" : ""}`.trim();
          if (studioTabVideoEl) studioTabVideoEl.className = `tabBtn ${!isAudio ? "tabBtnActive" : ""}`.trim();
          if (studioAudioPanelEl) studioAudioPanelEl.style.display = isAudio ? "block" : "none";
          if (studioVideoPanelEl) studioVideoPanelEl.style.display = isAudio ? "none" : "block";
        }

        if (studioTabAudioEl) studioTabAudioEl.addEventListener("click", () => setStudioTab("audio"));
        if (studioTabVideoEl) studioTabVideoEl.addEventListener("click", () => setStudioTab("video"));
        setStudioTab("audio");

        function buildFullPath() {
          const base = walletBasePathEl.value.trim();
          const chain = String(walletChainEl.value || "0").trim();
          const idx = String(walletAddrIndexEl.value || "0").trim();
          if (!base) return "";
          return `${base}/${chain}/${idx}`;
        }

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
            }
          } catch (_) {
            isWalletSessionUnlocked = false;
            updateSendPrivUi();
            updateStudioPrivUi();
            walletStatusEl.textContent = "Wallet error";
            walletUnlockedEl.textContent = "—";
          }
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
          if (!dialogApi?.save || !fsApi?.writeBinaryFile) {
            throw new Error("Save is not available (missing dialog/fs API)");
          }
          const path = await dialogApi.save({
            defaultPath: suggestedName,
            filters: filters || [],
          });
          if (!path) return null;
          const buf = await blob.arrayBuffer();
          const bytes = new Uint8Array(buf);
          await fsApi.writeBinaryFile({ path, contents: bytes });
          return String(path);
        }

        async function writeBlobToPath(blob, path) {
          if (!fsApi?.writeBinaryFile) {
            throw new Error("File write is not available (missing fs API)");
          }
          const buf = await blob.arrayBuffer();
          const bytes = new Uint8Array(buf);
          await fsApi.writeBinaryFile({ path, contents: bytes });
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

          const audioStream = await navigator.mediaDevices.getUserMedia({
            audio: micId ? { deviceId: { exact: micId } } : true,
            video: false,
          });

          let videoStream;
          if (source === "screen") {
            videoStream = await navigator.mediaDevices.getDisplayMedia({ video: true, audio: false });
          } else {
            videoStream = await navigator.mediaDevices.getUserMedia({
              video: camId ? { deviceId: { exact: camId } } : true,
              audio: false,
            });
          }

          const tracks = [...videoStream.getVideoTracks(), ...audioStream.getAudioTracks()];
          studioVideoStream = new MediaStream(tracks);

          if (studioVideoLiveEl) {
            studioVideoLiveEl.srcObject = studioVideoStream;
            studioVideoLiveEl.style.display = "block";
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
          }
        }

        if (studioAudioStartEl) {
          studioAudioStartEl.addEventListener("click", async () => {
            setError("");
            try {
              await refreshStudioDevices();
              await startStudioAudio();
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
              const out = await saveBlobToFile(studioAudioBlob, `kat_audio_${ts}.${ext}`, [
                { name: ext.toUpperCase(), extensions: [ext] },
              ]);
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
              const toAddress = studioAudioToEl?.value.trim() || "";
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
                      if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Sending…";
                      const ts = new Date().toISOString().replace(/[:.]/g, "-");
                      const ext = (studioAudioBlob.type || "").includes("ogg") ? "ogg" : "webm";
                      const fileName = `kat_audio_${ts}.${ext}`;
                      if (!studioAudioFilePath) {
                        studioAudioFilePath = await ensureStudioTempFilePath(studioAudioBlob, fileName);
                      }
                      const txid = await tauri.invoke("wallet_send_file_path", {
                        window: null,
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
                      if (studioAudioStatusEl) studioAudioStatusEl.textContent = "Sent";
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
              await refreshStudioDevices();
              await startStudioVideo();
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
              const out = await saveBlobToFile(studioVideoBlob, `kat_video_${ts}.webm`, [
                { name: "WEBM", extensions: ["webm"] },
              ]);
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
              const toAddress = studioVideoToEl?.value.trim() || "";
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
                      if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Sending…";
                      const ts = new Date().toISOString().replace(/[:.]/g, "-");
                      const fileName = `kat_video_${ts}.webm`;
                      if (!studioVideoFilePath) {
                        studioVideoFilePath = await ensureStudioTempFilePath(studioVideoBlob, fileName);
                      }
                      const txid = await tauri.invoke("wallet_send_file_path", {
                        window: null,
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
                      if (studioVideoStatusEl) studioVideoStatusEl.textContent = "Sent";
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
            const bal = await tauri.invoke("wallet_get_balance", {
              network,
              derivationPath: path,
              rpcUrl: rpcUrl || null,
            });
            const b = Number(bal);
            walletBalanceEl.textContent = Number.isFinite(b) ? b.toFixed(8) : "—";
          } catch (_) {
            walletBalanceEl.textContent = "—";
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
              actions: [{ label: "I saved it", primary: true }],
            });
          } catch (e) {
            setError(String(e));
          }
        });

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

        walletImportMnemonicBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const username = walletImportUsernameEl.value.trim();
            const password = walletImportPasswordEl.value;
            const phrase = walletImportMnemonicEl.value.trim().replace(/\s+/g, " ");
            const mnemonicPassword = walletMnemonicPassEl.value.trim();
            await tauri.invoke("wallet_profile_import_mnemonic", {
              username,
              password,
              phrase,
              mnemonicPassword: mnemonicPassword || null,
            });
            await refreshProfiles();
            showModal({
              title: "Imported",
              body: "Mnemonic profile imported.",
              actions: [{ label: "OK", primary: true }],
            });
          } catch (e) {
            setError(String(e));
          }
        });

        walletImportPrivKeyBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const username = walletImportUsernameEl.value.trim();
            const password = walletImportPasswordEl.value;
            const privateKeyHex = walletImportPrivKeyEl.value.trim();
            await tauri.invoke("wallet_profile_import_private_key", {
              username,
              password,
              privateKeyHex,
            });
            await refreshProfiles();
            showModal({
              title: "Imported",
              body: "Private key profile imported.",
              actions: [{ label: "OK", primary: true }],
            });
          } catch (e) {
            setError(String(e));
          }
        });

        walletUnlockBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            const username = walletProfileSelectEl.value.trim();
            const password = walletPasswordEl.value;
            await tauri.invoke("wallet_unlock", { username, password });
            await refreshWalletStatus();
            await refreshWalletAddressAndBalance();
          } catch (e) {
            setError(String(e));
          }
        });

        walletLockBtnEl.addEventListener("click", async () => {
          setError("");
          try {
            await tauri.invoke("wallet_lock", {});
            walletPasswordEl.value = "";
            walletProfileSelectEl.value = "";
            await refreshWalletStatus();
          } catch (e) {
            setError(String(e));
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
          try {
            const network = walletNetworkEl.value;
            const rpcUrl = walletRpcUrlEl.value.trim();
            const derivationPath = buildFullPath();
            const toAddress = walletSendToEl.value.trim();
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

        await refreshProfiles();
        await refreshWalletStatus();

        // Prevent default browser behavior (opening the file / navigation) on drop.
        window.addEventListener("dragover", (e) => e.preventDefault());
        window.addEventListener("drop", (e) => e.preventDefault());

        // Send panel drag & drop
        if (sendDropZoneEl) {
          const activate = () => sendDropZoneEl.classList.add("dropZoneActive");
          const deactivate = () => sendDropZoneEl.classList.remove("dropZoneActive");

          if (sendFileEl) {
            sendFileEl.addEventListener("change", () => {
              const f = sendFileEl.files && sendFileEl.files[0];
              setPickedFile(f || null);
            });
          }

          sendDropZoneEl.addEventListener("dragenter", (e) => {
            e.preventDefault();
            activate();
          });
          sendDropZoneEl.addEventListener("dragover", (e) => {
            e.preventDefault();
            activate();
          });
          sendDropZoneEl.addEventListener("dragleave", (e) => {
            e.preventDefault();
            deactivate();
          });
          sendDropZoneEl.addEventListener("drop", (e) => {
            e.preventDefault();
            deactivate();
            const f = e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0];
            if (f) {
              setError("");
              setDroppedFile(f);
            }
          });
        }

        if (eventApi?.listen) {
          await eventApi.listen("kaspa_send_progress", (ev) => {
            const p = ev?.payload || {};
            const done = Number(p.submitted_chunks || 0);
            const total = p.total_chunks == null ? null : Number(p.total_chunks);
            if (total && total > 0) {
              sendProgressEl.textContent = `${done}/${total}`;
              setRing(sendRingEl, done / total);
            } else {
              sendProgressEl.textContent = `${done}`;
              setRing(sendRingEl, 0);
            }
          });

          await eventApi.listen("kaspa_receive_progress", (ev) => {
            const p = ev?.payload || {};
            const found = Number(p.found_chunks || 0);
            const total = p.total_chunks == null ? null : Number(p.total_chunks);
            if (total && total > 0) {
              recvStatusEl.textContent = `${found}/${total}`;
              setRing(recvRingEl, found / total);
            } else {
              recvStatusEl.textContent = `${found}`;
              setRing(recvRingEl, 0);
            }
          });
        }

        sendBtn.addEventListener("click", async () => {
          setError("");
          sendTxidEl.textContent = "—";
          sendProgressEl.textContent = "Working…";
          setRing(sendRingEl, 0);

          const f = selectedSendFile || (sendFileEl.files && sendFileEl.files[0]);
          if (!f) {
            sendProgressEl.textContent = "Idle";
            setError("Select a file to send.");
            return;
          }

          const fromPrivateKey = sendPrivEl.value.trim();
          const toAddress = sendToEl.value.trim();
          const rpcUrl = sendRpcEl.value.trim();
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

          try {
            const fileB64 = await readFileB64(f);
            const txid = await tauri.invoke("wallet_send_file_b64", {
              window: null,
              accountId: null,
              fileB64,
              toAddress,
              amountKas,
              rpcUrl,
              resumeFrom: null,
              resumeOutputIndex: 1,
              fileName: f.name,
              fromPrivateKey,
            });
            sendTxidEl.textContent = String(txid);
            recvTxEl.value = String(txid);
            sendProgressEl.textContent = "Done";
            setRing(sendRingEl, 1);

            showModal({
              title: "Send complete",
              body: `Transaction ID:\n${txid}\n\nNext: open the explorer transaction and copy the first value under \"Block hashes\" to use as a scan anchor for receive.`
              ,
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

          try {
            const outPath = await tauri.invoke("wallet_receive_file", {
              window: null,
              txId,
              outputPath,
              rpcUrl,
              startBlockHash: startBlockHash || null,
            });
            lastReceivedPath = String(outPath);
            recvStatusEl.textContent = "Done";
            setRing(recvRingEl, 1);

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

      init();
    