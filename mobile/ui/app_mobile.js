(function () {
  const IS_ANDROID = /Android/i.test(String(navigator.userAgent || ""));
  const PREFERS_REDUCED_MOTION = (() => {
    try {
      return !!window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    } catch (_) {
      return false;
    }
  })();
  const IS_SMALL_SCREEN = (() => {
    try {
      return !!window.matchMedia && window.matchMedia("(max-width: 640px)").matches;
    } catch (_) {
      return false;
    }
  })();
  const DISABLE_HEAVY_ANIMATIONS = IS_ANDROID || PREFERS_REDUCED_MOTION || IS_SMALL_SCREEN;

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
  const sendTxidEl = document.getElementById("sendTxid");
  const sendProgressEl = document.getElementById("sendProgress");
  const sendRingEl = document.getElementById("sendRing");
  const sendCopyTxidBtnEl = document.getElementById("sendCopyTxidBtn");
  const sendExplorerLinkEl = document.getElementById("sendExplorerLink");

  const recvBtn = document.getElementById("recvBtn");
  const recvTxEl = document.getElementById("recvTx");
  const recvStartEl = document.getElementById("recvStart");
  const recvRpcEl = document.getElementById("recvRpc");
  const recvOutEl = document.getElementById("recvOut");
  const recvStatusEl = document.getElementById("recvStatus");
  const recvRingEl = document.getElementById("recvRing");
  const recvCopyTxBtnEl = document.getElementById("recvCopyTxBtn");
  const recvCopyStartBtnEl = document.getElementById("recvCopyStartBtn");
  const explorerTipEl = document.getElementById("explorerTip");
  const recvSavedPathEl = document.getElementById("recvSavedPath");
  const recvCopySavedPathBtnEl = document.getElementById("recvCopySavedPathBtn");

  const navWalletEl = document.getElementById("navWallet");

  const hdrWalletCreateEl = document.getElementById("hdrWalletCreate");
  const hdrWalletImportEl = document.getElementById("hdrWalletImport");
  const hdrWalletLoginEl = document.getElementById("hdrWalletLogin");
  const hdrWalletLogoutEl = document.getElementById("hdrWalletLogout");

  const walletStatusEl = document.getElementById("walletStatus");
  const walletUnlockedEl = document.getElementById("walletUnlocked");
  const walletProfileSelectEl = document.getElementById("walletProfileSelect");
  const walletRefreshBtnEl = document.getElementById("walletRefreshBtn");
  const walletPasswordEl = document.getElementById("walletPassword");
  const walletUnlockBtnEl = document.getElementById("walletUnlockBtn");
  const walletLockBtnEl = document.getElementById("walletLockBtn");

  const walletNetworkEl = document.getElementById("walletNetwork");
  const walletRpcUrlEl = document.getElementById("walletRpcUrl");
  const walletBasePathEl = document.getElementById("walletBasePath");
  const walletChainEl = document.getElementById("walletChain");
  const walletAddrIndexEl = document.getElementById("walletAddrIndex");
  const walletDeriveBtnEl = document.getElementById("walletDeriveBtn");
  const walletFullPathEl = document.getElementById("walletFullPath");
  const walletDerivedAddressEl = document.getElementById("walletDerivedAddress");

  const walletHeroBalanceEl = document.getElementById("walletHeroBalance");

  const walletBalanceEl = document.getElementById("walletBalance");
  const walletBalanceRefreshBtnEl = document.getElementById("walletBalanceRefreshBtn");

  const walletSendToEl = document.getElementById("walletSendTo");
  const walletSendAmountEl = document.getElementById("walletSendAmount");
  const walletSendKasBtnEl = document.getElementById("walletSendKasBtn");
  const walletSendKasTxidEl = document.getElementById("walletSendKasTxid");

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

  const walletTxHistoryStatusEl = document.getElementById("walletTxHistoryStatus");
  const walletTxReceivedListEl = document.getElementById("walletTxReceivedList");
  const walletTxSentListEl = document.getElementById("walletTxSentList");

  const walletReceiveQrSvgEl = document.getElementById("walletReceiveQrSvg");
  const walletReceiveQrFallbackEl = document.getElementById("walletReceiveQrFallback");
  const walletReceiveAddrTextEl = document.getElementById("walletReceiveAddrText");

  const walletCopyAddressBtnEl = document.getElementById("walletCopyAddressBtn");
  const walletCopySendKasTxidBtnEl = document.getElementById("walletCopySendKasTxidBtn");

  const clearSessionBtnEl = document.getElementById("clearSessionBtn");
  const openBtn = document.getElementById("openBtn");
  const revealBtn = document.getElementById("revealBtn");

  const RING_R = 18;
  const RING_CIRC = 2 * Math.PI * RING_R;

  function setError(msg) {
    if (!errorEl) return;
    if (!msg) {
      errorEl.style.display = "none";
      errorEl.textContent = "";
      return;
    }
    errorEl.style.display = "block";
    errorEl.textContent = String(msg);
  }

  function setRing(circleEl, pct) {
    const p = Number.isFinite(pct) ? Math.max(0, Math.min(1, pct)) : 0;
    if (!circleEl) return;
    circleEl.style.strokeDasharray = String(RING_CIRC);
    circleEl.style.strokeDashoffset = String(RING_CIRC * (1 - p));
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

  async function readFileB64(file) {
    return new Promise((resolve, reject) => {
      const r = new FileReader();
      r.onerror = () => reject(new Error("failed reading file"));
      r.onload = () => resolve(String(r.result || ""));
      r.readAsDataURL(file);
    });
  }

  function b64ToBlob(b64, mime) {
    const raw = String(b64 || "").trim();
    const payload = raw.includes(",") ? raw.split(",").slice(1).join(",") : raw;
    const bin = atob(payload);
    const bytes = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
    return new Blob([bytes], { type: mime || "application/octet-stream" });
  }

  function downloadBlob(blob, fileName) {
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = fileName || "received.bin";
    document.body.appendChild(a);
    a.click();
    a.remove();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }

  async function tryShareBlob(blob, fileName) {
    try {
      const name = String(fileName || "received.bin").trim() || "received.bin";
      const file = new File([blob], name, { type: blob?.type || "application/octet-stream" });
      if (!navigator?.share) return false;
      if (navigator?.canShare && !navigator.canShare({ files: [file] })) return false;
      await navigator.share({ files: [file], title: name });
      return true;
    } catch (_) {
      return false;
    }
  }

  function applyPublicOnlyUi() {
    const hide = (id) => {
      const el = document.getElementById(id);
      if (el) el.style.display = "none";
    };

    if (IS_ANDROID) {
      hide("globalRpcModeLocal");
      hide("sendRpcModeLocal");
      hide("recvRpcModeLocal");
      hide("studioAudioRpcModeLocal");
      hide("studioVideoRpcModeLocal");
      hide("walletRpcModeLocal");

      hide("navImport");
      hide("navStudio");

      // These buttons invoke missing commands on mobile.
      if (openBtn) openBtn.style.display = "none";
      if (revealBtn) revealBtn.style.display = "none";
    }

    const forcePublic = (inputId) => {
      const el = document.getElementById(inputId);
      if (!el) return;
      el.value = "public";
      try {
        el.dispatchEvent(new Event("input", { bubbles: true }));
      } catch (_) {}
    };

    forcePublic("sendRpc");
    forcePublic("recvRpc");
    forcePublic("studioAudioRpc");
    forcePublic("studioVideoRpc");
    forcePublic("walletRpcUrl");
  }

  function formatKas(v) {
    const n = Number(v);
    if (!Number.isFinite(n)) return "—";
    return n.toFixed(8).replace(/\.0+$/, "").replace(/(\.[0-9]*?)0+$/, "$1");
  }

  function ensureOpenWallet() {
    try {
      navWalletEl?.click();
    } catch (_) {}
  }

  function walletDerivationPath() {
    const base = String(walletBasePathEl?.value || "m/44'/111111'/0'/0");
    const chain = Number(walletChainEl?.value || 0);
    const idx = Number(walletAddrIndexEl?.value || 0);
    return `${base}/${chain}/${idx}`;
  }

  function inferNetworkFromRpcUrl(rpcUrl) {
    const v = String(rpcUrl || "").toLowerCase();
    if (v.includes("tn10") || v.includes("testnet")) return "testnet";
    if (v.includes("devnet")) return "devnet";
    return "mainnet";
  }

  async function walletRefreshProfiles(invoke) {
    if (!walletProfileSelectEl) return;
    const prev = walletProfileSelectEl.value;
    const profiles = (await invoke("wallet_profiles_list", {})) || [];
    walletProfileSelectEl.innerHTML = "";
    const opt0 = document.createElement("option");
    opt0.value = "";
    opt0.textContent = "(none)";
    walletProfileSelectEl.appendChild(opt0);
    profiles.forEach((p) => {
      const o = document.createElement("option");
      o.value = p.username;
      o.textContent = p.username;
      walletProfileSelectEl.appendChild(o);
    });
    if (prev) walletProfileSelectEl.value = prev;
  }

  async function walletRefreshStatus(invoke) {
    try {
      const u = await invoke("wallet_unlocked_username", {});
      const unlocked = u ? String(u) : "";
      if (walletStatusEl) walletStatusEl.textContent = unlocked ? "Unlocked" : "Locked";
      if (walletUnlockedEl) walletUnlockedEl.textContent = unlocked || "—";

      if (sendPrivAutoHintEl) sendPrivAutoHintEl.style.display = unlocked ? "block" : "none";

      return !!unlocked;
    } catch (_) {
      if (walletStatusEl) walletStatusEl.textContent = "Wallet error";
      if (walletUnlockedEl) walletUnlockedEl.textContent = "—";
      if (sendPrivAutoHintEl) sendPrivAutoHintEl.style.display = "none";
      return false;
    }
  }

  async function walletRefreshAddressAndBalance(invoke) {
    if (!walletNetworkEl) return;
    const network = String(walletNetworkEl.value || "mainnet");
    const rpcUrl = String(walletRpcUrlEl?.value || "public").trim() || "public";
    const path = walletDerivationPath();

    if (walletFullPathEl) walletFullPathEl.textContent = path;

    const addr = await invoke("wallet_derive_receive_address", { network, derivationPath: path });
    const addrStr = String(addr || "").trim();
    if (walletDerivedAddressEl) walletDerivedAddressEl.textContent = addrStr || "—";
    if (walletReceiveAddrTextEl) walletReceiveAddrTextEl.textContent = addrStr || "—";

    try {
      const svgStr = await invoke("wallet_qr_svg", { data: addrStr });
      if (walletReceiveQrSvgEl) {
        walletReceiveQrSvgEl.innerHTML = String(svgStr || "");
        walletReceiveQrSvgEl.style.display = "block";
      }
      if (walletReceiveQrFallbackEl) walletReceiveQrFallbackEl.style.display = "none";
    } catch (_) {
      if (walletReceiveQrSvgEl) walletReceiveQrSvgEl.innerHTML = "";
      if (walletReceiveQrFallbackEl) walletReceiveQrFallbackEl.style.display = "block";
    }

    if (walletBalanceEl) walletBalanceEl.textContent = "…";
    if (walletHeroBalanceEl) walletHeroBalanceEl.textContent = "…";
    const bal = await invoke("wallet_get_balance", { network, derivationPath: path, rpcUrl });
    const balStr = formatKas(bal);
    if (walletBalanceEl) walletBalanceEl.textContent = balStr;
    if (walletHeroBalanceEl) walletHeroBalanceEl.textContent = balStr;
  }

  function renderTxList(container, items, kind) {
    if (!container) return;
    container.innerHTML = "";
    (items || []).slice(0, 20).forEach((t) => {
      const row = document.createElement("div");
      row.className = "txItem";
      const txid = String(t.txid || "");
      const kas = Number(t.netSompi || t.net_sompi || 0) / 100000000;
      row.innerHTML = `<div style="font-weight:850;">${kind} ${formatKas(Math.abs(kas))} KAS</div><div style="font-size:11px;opacity:0.7;word-break:break-all;">${txid}</div>`;
      container.appendChild(row);
    });
  }

  async function walletRefreshTxHistory(invoke) {
    const addr = String(walletDerivedAddressEl?.textContent || "").trim();
    if (!walletNetworkEl) return;
    if (!addr || addr === "—") {
      if (walletTxHistoryStatusEl) walletTxHistoryStatusEl.textContent = "Derive an address to view history.";
      if (walletTxReceivedListEl) walletTxReceivedListEl.innerHTML = "";
      if (walletTxSentListEl) walletTxSentListEl.innerHTML = "";
      return;
    }
    if (walletTxHistoryStatusEl) walletTxHistoryStatusEl.textContent = "Loading…";
    const network = String(walletNetworkEl.value || "mainnet");
    const items = (await invoke("wallet_tx_history", { network, address: addr, limit: 50, offset: 0 })) || [];

    const received = [];
    const sent = [];
    items.forEach((t) => {
      const net = Number(t.netSompi ?? t.net_sompi ?? 0);
      if (net >= 0) received.push(t);
      else sent.push(t);
    });
    if (walletTxHistoryStatusEl) walletTxHistoryStatusEl.textContent = "";
    renderTxList(walletTxReceivedListEl, received, "+");
    renderTxList(walletTxSentListEl, sent, "-");
  }

  function hideSplashFast() {
    const splashEl = document.getElementById("splash");
    const splashCanvasEl = document.getElementById("splashCanvas");
    const splashNodesEl = document.getElementById("splashNodes");
    if (!splashEl) return;

    if (DISABLE_HEAVY_ANIMATIONS) {
      splashEl.classList.add("splashFade");
      splashEl.setAttribute("aria-hidden", "true");
      if (splashCanvasEl) splashCanvasEl.style.display = "none";
      if (splashNodesEl) splashNodesEl.innerHTML = "";
      return;
    }

    setTimeout(() => {
      splashEl.classList.add("splashFade");
      splashEl.setAttribute("aria-hidden", "true");
    }, 700);
  }

  async function main() {
    hideSplashFast();
    applyPublicOnlyUi();

    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.tauri?.invoke;
    if (!invoke) {
      setError("Tauri API not available.");
      return;
    }

    if (!sendBtn || !recvBtn) {
      setError("UI elements missing.");
      return;
    }

    // Wallet wiring
    if (hdrWalletLoginEl) {
      hdrWalletLoginEl.addEventListener("click", () => {
        try {
          walletPasswordEl?.focus();
        } catch (_) {}
      });
    }
    if (hdrWalletLogoutEl) {
      hdrWalletLogoutEl.addEventListener("click", async () => {
        try {
          await invoke("wallet_lock", {});
          await walletRefreshStatus(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }
    if (walletRefreshBtnEl) {
      walletRefreshBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          await walletRefreshProfiles(invoke);
          await walletRefreshStatus(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletUnlockBtnEl) {
      walletUnlockBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          const username = String(walletProfileSelectEl?.value || "").trim();
          const password = String(walletPasswordEl?.value || "");
          await invoke("wallet_unlock", { username, password });
          await walletRefreshStatus(invoke);
          await walletRefreshAddressAndBalance(invoke);
          await walletRefreshTxHistory(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletLockBtnEl) {
      walletLockBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          await invoke("wallet_lock", {});
          if (walletPasswordEl) walletPasswordEl.value = "";
          if (walletProfileSelectEl) walletProfileSelectEl.value = "";
          await walletRefreshStatus(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletDeriveBtnEl) {
      walletDeriveBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          await walletRefreshAddressAndBalance(invoke);
          await walletRefreshTxHistory(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }
    if (walletBalanceRefreshBtnEl) {
      walletBalanceRefreshBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          await walletRefreshAddressAndBalance(invoke);
          await walletRefreshTxHistory(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletSendKasBtnEl) {
      walletSendKasBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          const network = String(walletNetworkEl?.value || "mainnet");
          const rpcUrl = String(walletRpcUrlEl?.value || "public").trim() || "public";
          const derivationPath = walletDerivationPath();
          const toAddress = String(walletSendToEl?.value || "").trim();
          const amountKas = Number(walletSendAmountEl?.value || "0");
          if (!toAddress) throw new Error("Enter a destination address.");
          if (!Number.isFinite(amountKas) || amountKas <= 0) throw new Error("Enter an amount > 0.");
          if (walletSendKasTxidEl) walletSendKasTxidEl.textContent = "Working…";
          const txid = await invoke("wallet_send_kas", { network, derivationPath, rpcUrl, toAddress, amountKas });
          if (walletSendKasTxidEl) walletSendKasTxidEl.textContent = String(txid);
          await walletRefreshAddressAndBalance(invoke);
          await walletRefreshTxHistory(invoke);
        } catch (e) {
          if (walletSendKasTxidEl) walletSendKasTxidEl.textContent = "—";
          setError(String(e));
        }
      });
    }

    if (walletCopyAddressBtnEl) {
      walletCopyAddressBtnEl.addEventListener("click", async () => {
        try {
          await copyText(walletDerivedAddressEl?.textContent);
        } catch (e) {
          setError(String(e));
        }
      });
    }
    if (walletCopySendKasTxidBtnEl) {
      walletCopySendKasTxidBtnEl.addEventListener("click", async () => {
        try {
          await copyText(walletSendKasTxidEl?.textContent);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletCreateBtnEl) {
      walletCreateBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          const username = String(walletNewUsernameEl?.value || "").trim();
          const password = String(walletNewPasswordEl?.value || "");
          const wordCount = Number(walletWordCountEl?.value || "24");
          const phrase = await invoke("wallet_profile_create_mnemonic", { username, password, wordCount });
          await walletRefreshProfiles(invoke);
          await copyText(String(phrase));
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletImportMnemonicBtnEl) {
      walletImportMnemonicBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          const username = String(walletImportUsernameEl?.value || "").trim();
          const password = String(walletImportPasswordEl?.value || "");
          const phrase = String(walletImportMnemonicEl?.value || "").trim();
          const mnemonicPasswordRaw = String(walletMnemonicPassEl?.value || "");
          const mnemonicPassword = mnemonicPasswordRaw.trim() ? mnemonicPasswordRaw : null;
          await invoke("wallet_profile_import_mnemonic", { username, password, phrase, mnemonicPassword });
          await walletRefreshProfiles(invoke);
        } catch (e) {
          setError(String(e));
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
          await invoke("wallet_profile_import_private_key", { username, password, privateKeyHex });
          await walletRefreshProfiles(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletDeleteProfileBtnEl) {
      walletDeleteProfileBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          const username = String(walletProfileSelectEl?.value || "").trim();
          if (!username) throw new Error("Select a profile to delete.");
          await invoke("wallet_profile_delete", { username });
          await walletRefreshProfiles(invoke);
          await walletRefreshStatus(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (walletClearAllProfilesBtnEl) {
      walletClearAllProfilesBtnEl.addEventListener("click", async () => {
        setError("");
        try {
          await invoke("wallet_profiles_clear_all", {});
          await walletRefreshProfiles(invoke);
          await walletRefreshStatus(invoke);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    // Initial wallet state
    try {
      await walletRefreshProfiles(invoke);
      await walletRefreshStatus(invoke);
    } catch (_) {}

    if (sendDropZoneEl) {
      sendDropZoneEl.addEventListener("dragover", (e) => {
        e.preventDefault();
      });
      sendDropZoneEl.addEventListener("drop", (e) => {
        e.preventDefault();
        const f = e.dataTransfer?.files?.[0];
        if (!f) return;
        if (sendFileEl) sendFileEl.files = e.dataTransfer.files;
        if (sendFileHintEl) sendFileHintEl.textContent = `Selected: ${f.name}`;
      });
    }

    if (sendFileEl) {
      sendFileEl.addEventListener("change", () => {
        const f = sendFileEl.files?.[0];
        if (!f) return;
        if (sendFileHintEl) sendFileHintEl.textContent = `Selected: ${f.name}`;
      });
    }

    if (sendExplorerLinkEl) {
      sendExplorerLinkEl.addEventListener("click", async (e) => {
        e.preventDefault();
        await openExternal(getExplorerTxUrl(sendTxidEl?.textContent));
      });
    }

    if (sendCopyTxidBtnEl) {
      sendCopyTxidBtnEl.addEventListener("click", async () => {
        try {
          await copyText(String(sendTxidEl?.textContent || "").trim());
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (recvCopyTxBtnEl) {
      recvCopyTxBtnEl.addEventListener("click", async () => {
        try {
          await copyText(recvTxEl?.value);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (recvCopyStartBtnEl) {
      recvCopyStartBtnEl.addEventListener("click", async () => {
        try {
          await copyText(recvStartEl?.value);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (recvCopySavedPathBtnEl) {
      recvCopySavedPathBtnEl.addEventListener("click", async () => {
        try {
          await copyText(recvSavedPathEl?.textContent);
        } catch (e) {
          setError(String(e));
        }
      });
    }

    if (explorerTipEl) {
      explorerTipEl.addEventListener("click", async (e) => {
        e.preventDefault();
        await openExternal(getExplorerTxUrl(recvTxEl?.value));
      });
    }

    if (clearSessionBtnEl) {
      clearSessionBtnEl.addEventListener("click", () => {
        setError("");
        if (sendTxidEl) sendTxidEl.textContent = "—";
        if (sendProgressEl) sendProgressEl.textContent = "Idle";
        if (sendToEl) sendToEl.value = "";
        if (sendPrivEl) sendPrivEl.value = "";
        if (sendAmountEl) sendAmountEl.value = "0.15";
        if (recvTxEl) recvTxEl.value = "";
        if (recvStartEl) recvStartEl.value = "";
        if (recvOutEl) recvOutEl.value = "received.bin";
        if (recvStatusEl) recvStatusEl.textContent = "Idle";
        if (recvSavedPathEl) recvSavedPathEl.textContent = "—";
        setRing(sendRingEl, 0);
        setRing(recvRingEl, 0);
      });
    }

    sendBtn.addEventListener("click", async () => {
      setError("");
      if (sendTxidEl) sendTxidEl.textContent = "—";
      if (sendProgressEl) sendProgressEl.textContent = "Working…";
      setRing(sendRingEl, 0);

      try {
        const f = sendFileEl?.files?.[0];
        if (!f) throw new Error("Select a file to send.");

        const toAddress = String(sendToEl?.value || "").trim();
        const amountKas = Number(sendAmountEl?.value || "0");
        const rpcUrl = String(sendRpcEl?.value || "public").trim() || "public";
        let fromPrivateKey = String(sendPrivEl?.value || "").trim();

        if (!toAddress) throw new Error("Enter to address.");
        if (!Number.isFinite(amountKas) || amountKas <= 0) throw new Error("Enter an amount > 0.");
        if (!fromPrivateKey) {
          const network = inferNetworkFromRpcUrl(rpcUrl);
          const derivationPath = walletDerivationPath();
          try {
            fromPrivateKey = await invoke("wallet_unlocked_private_key_hex", {
              network,
              derivationPath,
            });
            fromPrivateKey = String(fromPrivateKey || "").trim();
          } catch (_) {
            fromPrivateKey = "";
          }
        }

        if (!fromPrivateKey) throw new Error("Enter from private key (hex) or unlock wallet to sign automatically.");

        const fileB64 = await readFileB64(f);
        const txid = await invoke("wallet_send_file_b64", {
          fileB64,
          toAddress,
          amountKas,
          rpcUrl,
          resumeFrom: null,
          resumeOutputIndex: 1,
          fromPrivateKey,
        });

        if (sendTxidEl) sendTxidEl.textContent = String(txid);
        if (sendProgressEl) sendProgressEl.textContent = "Done";
        setRing(sendRingEl, 1);
      } catch (e) {
        if (sendProgressEl) sendProgressEl.textContent = "Failed";
        setRing(sendRingEl, 0);
        setError(String(e));
      }
    });

    recvBtn.addEventListener("click", async () => {
      setError("");
      if (recvStatusEl) recvStatusEl.textContent = "Working…";
      setRing(recvRingEl, 0);
      if (recvSavedPathEl) recvSavedPathEl.textContent = "—";

      try {
        const txId = String(recvTxEl?.value || "").trim();
        const startBlockHash = String(recvStartEl?.value || "").trim() || null;
        const rpcUrl = String(recvRpcEl?.value || "public").trim() || "public";

        if (!txId) throw new Error("Enter a transaction id.");
        const outputName = String(recvOutEl?.value || "").trim() || "received.bin";

        if (IS_ANDROID) {
          const dialog = window.__TAURI__?.dialog;
          const fs = window.__TAURI__?.fs;
          if (dialog?.save && fs?.writeFile) {
            try {
              const ext = String(outputName).split(".").pop();
              const pathOrUri = await dialog.save({
                title: "Save received file",
                defaultPath: outputName,
                filters: ext
                  ? [{ name: "File", extensions: [String(ext)] }]
                  : [{ name: "File", extensions: ["bin"] }],
              });

              if (pathOrUri) {
                const b64 = await invoke("wallet_receive_file_b64", {
                  txId,
                  rpcUrl,
                  startBlockHash,
                });

                const bytes = Uint8Array.from(atob(String(b64 || "")), (c) => c.charCodeAt(0));
                await fs.writeFile(pathOrUri, bytes);

                if (recvSavedPathEl) recvSavedPathEl.textContent = String(pathOrUri);
                if (recvStatusEl) recvStatusEl.textContent = "Saved";
                setRing(recvRingEl, 1);
                return;
              }
            } catch (_) {
              // fall through to Rust-side save
            }
          }

          let savedPath;
          try {
            savedPath = await invoke("wallet_receive_file_save_downloads", {
              txId,
              rpcUrl,
              startBlockHash,
              outputName,
            });
          } catch (_) {
            savedPath = await invoke("wallet_receive_file_save", {
              txId,
              rpcUrl,
              startBlockHash,
              outputName,
            });
          }
          if (recvSavedPathEl) recvSavedPathEl.textContent = String(savedPath || "").trim() || "—";
          if (recvStatusEl) recvStatusEl.textContent = "Saved";
        } else {
          const b64 = await invoke("wallet_receive_file_b64", {
            txId,
            rpcUrl,
            startBlockHash,
          });

          const blob = b64ToBlob(String(b64 || ""), "application/octet-stream");
          const shared = await tryShareBlob(blob, outputName);
          if (!shared) downloadBlob(blob, outputName);

          if (recvStatusEl) recvStatusEl.textContent = shared ? "Shared" : "Done";
        }
        setRing(recvRingEl, 1);
      } catch (e) {
        if (recvStatusEl) recvStatusEl.textContent = "Failed";
        setRing(recvRingEl, 0);
        setError(String(e));
      }
    });
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

  document.addEventListener("DOMContentLoaded", () => {
    void main();
  });
})();
