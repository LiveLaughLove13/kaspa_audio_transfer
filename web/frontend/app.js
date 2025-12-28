document.addEventListener("DOMContentLoaded", () => {
  const dropZone = document.getElementById("dropZone");
  const fileInput = document.getElementById("fileInput");
  const estimateBtn = document.getElementById("estimateBtn");
  const statusMessage = document.getElementById("statusMessage");
  const estimateForm = document.getElementById("estimateForm");
  const appRefreshBtn = document.getElementById("appRefreshBtn");

  const howToModal = document.getElementById("howToModal");
  const howToCopyBtn = document.getElementById("howToCopyBtn");
  const howToCopyText = document.getElementById("howToCopyText");

  const fieldHelpModal = document.getElementById("fieldHelpModal");
  const fieldHelpModalTitle = document.getElementById("fieldHelpModalTitle");
  const fieldHelpModalSubtitle = document.getElementById("fieldHelpModalSubtitle");
  const fieldHelpCopyBtn = document.getElementById("fieldHelpCopyBtn");
  const fieldHelpCopyText = document.getElementById("fieldHelpCopyText");
  const fieldHelpList = document.getElementById("fieldHelpList");

  const payerAddress = document.getElementById("payerAddress");
  const fromPrivateKey = document.getElementById("fromPrivateKey");
  const storageAmountKas = document.getElementById("storageAmountKas");
  const rpcUrl = document.getElementById("rpcUrl");

  const sendToAddress = document.getElementById("sendToAddress");
  const sendAmountKas = document.getElementById("sendAmountKas");
  const sendResumeFrom = document.getElementById("sendResumeFrom");
  const sendResumeOutputIndex = document.getElementById("sendResumeOutputIndex");
  const sendUsePaymentAddress = document.getElementById("sendUsePaymentAddress");
  const sendBtn = document.getElementById("sendBtn");
  const sendTxId = document.getElementById("sendTxId");
  const sendProgressText = document.getElementById("sendProgressText");
  const sendProgressBar = document.getElementById("sendProgressBar");
  const sendProgressRing = document.getElementById("sendProgressRing");
  const sendProgressRingValue = document.getElementById("sendProgressRingValue");
  const sendProgressRingLabel = document.getElementById("sendProgressRingLabel");
  const sendElapsed = document.getElementById("sendElapsed");
  const sendEta = document.getElementById("sendEta");

  const receiveTxId = document.getElementById("receiveTxId");
  const receiveStartBlockHash = document.getElementById("receiveStartBlockHash");
  const receiveOutputName = document.getElementById("receiveOutputName");
  const receiveBtn = document.getElementById("receiveBtn");
  const receiveDownloadedInfo = document.getElementById("receiveDownloadedInfo");
  const receiveElapsed = document.getElementById("receiveElapsed");

  const libraryRefreshBtn = document.getElementById("libraryRefreshBtn");
  const libraryBaseDir = document.getElementById("libraryBaseDir");
  const libraryEmpty = document.getElementById("libraryEmpty");
  const libraryList = document.getElementById("libraryList");

  const resultSection = document.getElementById("resultSection");
  const resultFileName = document.getElementById("resultFileName");
  const resultFileSize = document.getElementById("resultFileSize");
  const resultChunkCount = document.getElementById("resultChunkCount");
  const resultTotalFee = document.getElementById("resultTotalFee");
  const resultEffectiveCost = document.getElementById("resultEffectiveCost");
  const resultExpectedAmount = document.getElementById("resultExpectedAmount");
  const resultPaymentAddress = document.getElementById("resultPaymentAddress");
  const resultJobId = document.getElementById("resultJobId");
  const resultStatus = document.getElementById("resultStatus");
  const dropZoneText = document.getElementById("dropZoneText");

  let selectedFile = null;
  let currentJobId = null;
  let pollTimer = null;
  let lastEstimate = null;

  let sendTimerStart = null;
  let sendTimerInterval = null;
  let receiveTimerStart = null;
  let receiveTimerInterval = null;

  estimateForm?.addEventListener("submit", (e) => {
    e.preventDefault();
  });

  function initCollapsibleCards() {
    const cards = Array.from(document.querySelectorAll(".card[data-card-id]"));
    for (const card of cards) {
      const id = card.getAttribute("data-card-id") || "";
      const toggle = card.querySelector(".card-toggle");
      const body = card.querySelector(".card-body");
      if (!(toggle instanceof HTMLButtonElement) || !(body instanceof HTMLElement)) continue;

      const key = id ? `kat_card_collapsed_${id}` : "";
      const stored = key ? localStorage.getItem(key) : null;
      const shouldCollapse = stored === "1";

      const apply = (collapsed) => {
        card.classList.toggle("card--collapsed", collapsed);
        toggle.setAttribute("aria-expanded", collapsed ? "false" : "true");

        if (collapsed) {
          body.style.maxHeight = "0px";
        } else {
          body.style.maxHeight = body.scrollHeight + "px";
        }

        if (key) localStorage.setItem(key, collapsed ? "1" : "0");
      };

      const syncExpandedHeight = () => {
        if (!card.classList.contains("card--collapsed")) {
          body.style.maxHeight = body.scrollHeight + "px";
        }
      };

      apply(shouldCollapse);

      toggle.addEventListener("click", (e) => {
        e.preventDefault();
        e.stopPropagation();
        const collapsed = !card.classList.contains("card--collapsed");
        apply(collapsed);
      });

      window.addEventListener("resize", syncExpandedHeight);
    }
  }

  initCollapsibleCards();

  function initSidebarNav() {
    const scroller = document.getElementById("mainScroll");
    const navItems = Array.from(document.querySelectorAll(".nav-item[data-target]"));
    const cards = Array.from(document.querySelectorAll(".card[data-card-id]"));
    if (!(scroller instanceof HTMLElement) || navItems.length === 0 || cards.length === 0) return;

    const byId = new Map();
    for (const c of cards) {
      const id = c.getAttribute("data-card-id") || "";
      if (id) byId.set(id, c);
    }

    const key = "kat_active_section";
    const stored = localStorage.getItem(key);
    const defaultId = byId.has("choose") ? "choose" : (byId.keys().next().value || "");
    let activeId = stored && stored !== "next" && byId.has(stored) ? stored : defaultId;

    const setActiveButton = (id) => {
      for (const b of navItems) {
        b.classList.toggle("active", b.getAttribute("data-target") === id);
      }
    };

    const showOnly = (id) => {
      for (const [cid, card] of byId.entries()) {
        const shouldHide = cid !== id;
        card.classList.toggle("view-hidden", shouldHide);
      }
      setActiveButton(id);
      localStorage.setItem(key, id);

      const card = byId.get(id);
      if (card) {
        const body = card.querySelector(".card-body");
        if (body instanceof HTMLElement && !card.classList.contains("card--collapsed")) {
          body.style.maxHeight = body.scrollHeight + "px";
        }
      }

      scroller.scrollTop = 0;
    };

    const isModalOpen = () => !!(howToModal && !howToModal.classList.contains("hidden"));

    const isFieldHelpOpen = () => !!(fieldHelpModal && !fieldHelpModal.classList.contains("hidden"));

    const syncBodyScrollLock = () => {
      const anyOpen = isModalOpen() || isFieldHelpOpen();
      document.body.style.overflow = anyOpen ? "hidden" : "";
    };

    const closeHowToModal = () => {
      if (!howToModal) return;
      howToModal.classList.add("hidden");
      syncBodyScrollLock();
    };

    const openHowToModal = () => {
      if (!howToModal) return;
      howToModal.classList.remove("hidden");
      syncBodyScrollLock();
    };

    const closeFieldHelpModal = () => {
      if (!fieldHelpModal) return;
      fieldHelpModal.classList.add("hidden");
      syncBodyScrollLock();
    };

    const openFieldHelpModal = () => {
      if (!fieldHelpModal) return;
      fieldHelpModal.classList.remove("hidden");
      syncBodyScrollLock();
    };

    if (howToModal) {
      howToModal.addEventListener("click", (e) => {
        const t = e.target;
        if (!(t instanceof HTMLElement)) return;
        if (t.hasAttribute("data-modal-close") || t.closest("[data-modal-close]")) {
          closeHowToModal();
        }
      });
    }

    document.addEventListener("keydown", (e) => {
      if (e.key !== "Escape") return;
      if (isFieldHelpOpen()) {
        closeFieldHelpModal();
        return;
      }
      if (isModalOpen()) {
        closeHowToModal();
      }
    });

    howToCopyBtn?.addEventListener("click", async () => {
      const raw = howToCopyText ? howToCopyText.textContent || "" : "";
      if (!raw.trim()) return;
      try {
        await navigator.clipboard.writeText(raw);
        setStatus("Copied How-to to clipboard.");
      } catch {
        setStatus("Copy failed. Please copy manually.");
      }
    });

    const helpContent = {
      payerAddress: {
        title: "Your Kaspa address (Kaspa:)",
        subtitle: "This is the address that pays network fees for the estimate and for sending. It must match the network prefix your node is on.",
        example: "kaspa:qq...",
        notes: [
          "Use a valid kaspa: address (mainnet/testnet prefix must match your node).",
          "This address is used to query UTXOs; some nodes require UTXO index support.",
        ],
      },
      fromPrivateKey: {
        title: "Private key (Required)",
        subtitle: "The private key that controls the payer address. It is used to sign transactions. Keep it secret.",
        example: "1234567891234567891234567891236512345678912345678912345678912365",
        notes: [
          "Never share this key. Anyone with it can spend your funds.",
          "Paste the 64-hex-character private key (32 bytes).",
        ],
      },
      storageAmountKas: {
        title: "Storage amount (KAS)",
        subtitle: "Optional payment amount sent in the manifest transaction (separate from network fees).",
        example: "0.2",
        notes: [
          "Network fees are additional and shown in the estimate.",
          "Set to 0 if you only want to store data without paying a recipient.",
        ],
      },
      rpcUrl: {
        title: "RPC URL (optional)",
        subtitle: "Kaspa gRPC endpoint used to talk to your node. If blank, the default is used.",
        example: "grpc://127.0.0.1:16110",
        notes: [
          "Use a local node for best reliability.",
          "If using a remote node, make sure it is synced and reachable.",
        ],
      },
      sendToAddress: {
        title: "To address",
        subtitle: "Recipient Kaspa address. The manifest transaction can include a payment to this address.",
        example: "kaspa:qq...",
        notes: [
          "You can auto-fill this from the estimate using the checkbox above.",
          "Prefix must match the node network (mainnet/testnet).",
        ],
      },
      sendAmountKas: {
        title: "Amount (KAS)",
        subtitle: "Amount to send to the recipient address in the manifest transaction.",
        example: "0.2",
        notes: [
          "This is separate from network fees.",
          "If you set 0, only change/output fees will apply (depending on flow).",
        ],
      },
      sendResumeFrom: {
        title: "Resume from (optional)",
        subtitle: "If a previous upload was interrupted, provide the manifest txid to resume chunk submission.",
        example: "b0c3220031a009c0b8bf71411acab1657ca8680b535bcee704f3e9e80939d6c1",
        notes: [
          "Use this together with Resume output index.",
          "Only use if you know the previous chain output is still unspent/available.",
        ],
      },
      sendResumeOutputIndex: {
        title: "Resume output index",
        subtitle: "The output index (vout) to resume spending from when continuing an interrupted upload.",
        example: "1",
        notes: [
          "Default is usually 1 when there is a payment output and a change output.",
          "If amount is 0, the change output index may be 0.",
        ],
      },
      receiveTxId: {
        title: "Transaction ID",
        subtitle: "The manifest transaction id (txid) returned after sending. This is what you use to download the file.",
        example: "b0c3220031a009c0b8bf71411acab1657ca8680b535bcee704f3e9e80939d6c1",
        notes: [
          "Paste the exact txid shown after a successful send.",
          "If you paste a chunk txid instead, only that chunk payload will be returned.",
        ],
      },
      receiveStartBlockHash: {
        title: "Start block hash (optional)",
        subtitle: "Optional optimization to start scanning from a known block hash to find chunks faster.",
        example: "eb92329b04ffe0bd70357b365e50309c9daee8cb8751d26933b62da5283840fc",
        notes: [
          "Best value is the transaction's accepting block hash (a 64-hex block hash).",
          "Useful if the tx is old and scanning from pruning point is slow.",
          "Leave blank if you are unsure.",
        ],
      },
      receiveOutputName: {
        title: "Output name (required)",
        subtitle: "The filename to save the downloaded audio/data as.",
        example: "received.mp3",
        notes: [
          "Include an extension (e.g. .mp3) so your OS knows how to open it.",
          "The file will appear in the Library section after download.",
        ],
      },
    };

    const renderFieldHelp = (key) => {
      const cfg = helpContent[key];
      if (!cfg) return;
      if (fieldHelpModalTitle) fieldHelpModalTitle.textContent = cfg.title;
      if (fieldHelpModalSubtitle) fieldHelpModalSubtitle.textContent = cfg.subtitle;
      if (fieldHelpCopyText) fieldHelpCopyText.textContent = cfg.example || "";
      if (fieldHelpList) {
        fieldHelpList.innerHTML = "";
        for (const line of cfg.notes || []) {
          const li = document.createElement("li");
          li.textContent = line;
          fieldHelpList.appendChild(li);
        }
      }
    };

    const helpButtons = Array.from(document.querySelectorAll(".field-help[data-help-key]"));
    for (const b of helpButtons) {
      b.addEventListener("click", () => {
        const key = b.getAttribute("data-help-key") || "";
        if (!key) return;
        if (!fieldHelpModal) {
          setStatus("Help popup unavailable (missing #fieldHelpModal). Try refreshing.");
          return;
        }
        renderFieldHelp(key);
        openFieldHelpModal();
      });
    }

    const receiveTxExplorerBtn = document.getElementById("receiveTxExplorerBtn");
    receiveTxExplorerBtn?.addEventListener("click", () => {
      const txid = (receiveTxId?.value || "").trim();
      if (!txid) {
        setStatus("Paste a Transaction ID first, then click Explorer.");
        return;
      }

      if (!fieldHelpModal) {
        setStatus("Help popup unavailable (missing #fieldHelpModal). Try refreshing.");
        return;
      }

      if (fieldHelpModalTitle) fieldHelpModalTitle.textContent = "Transaction ID (Explorer)";

      const explorerUrl = `https://explorer.kaspa.org/transactions/${encodeURIComponent(txid)}`;
      if (fieldHelpModalSubtitle) {
        fieldHelpModalSubtitle.textContent = "";
        const prefix = document.createTextNode("Open in explorer: ");
        const a = document.createElement("a");
        a.href = explorerUrl;
        a.target = "_blank";
        a.rel = "noopener noreferrer";
        a.textContent = explorerUrl;
        fieldHelpModalSubtitle.appendChild(prefix);
        fieldHelpModalSubtitle.appendChild(a);
      }

      if (fieldHelpCopyText) fieldHelpCopyText.textContent = txid;

      if (fieldHelpList) {
        fieldHelpList.innerHTML = "";
        const notes = [
          "Click the explorer link above.",
          "In the transaction page, locate the 'Block hashes' section.",
          "Copy the accepting block hash (64 hex characters).",
          "Paste the hash into 'Start block hash (optional)' and then download.",
        ];
        for (const line of notes) {
          const li = document.createElement("li");
          li.textContent = line;
          fieldHelpList.appendChild(li);
        }
      }

      openFieldHelpModal();
    });

    if (fieldHelpModal) {
      fieldHelpModal.addEventListener("click", (e) => {
        const t = e.target;
        if (!(t instanceof HTMLElement)) return;
        if (t.hasAttribute("data-fieldhelp-close") || t.closest("[data-fieldhelp-close]")) {
          closeFieldHelpModal();
        }
      });
    }

    fieldHelpCopyBtn?.addEventListener("click", async () => {
      const raw = fieldHelpCopyText ? fieldHelpCopyText.textContent || "" : "";
      if (!raw.trim()) return;
      try {
        await navigator.clipboard.writeText(raw);
        setStatus("Copied example to clipboard.");
      } catch {
        setStatus("Copy failed. Please copy manually.");
      }
    });

    for (const btn of navItems) {
      btn.addEventListener("click", () => {
        const id = btn.getAttribute("data-target") || "";
        if (!id) return;

        if (id === "next") {
          if (!howToModal) {
            setStatus("How-to popup is unavailable (missing #howToModal). Try refreshing.");
            return;
          }
          openHowToModal();
          setActiveButton(activeId);
          return;
        }

        if (!byId.has(id)) return;
        activeId = id;
        showOnly(activeId);
      });
    }

    showOnly(activeId);
  }

  initSidebarNav();

  appRefreshBtn?.addEventListener("click", () => {
    resetSession();
  });

  sendUsePaymentAddress?.addEventListener("change", () => {
    if (!sendToAddress) return;
    if (!sendUsePaymentAddress.checked) return;
    const addr = lastEstimate && typeof lastEstimate.payment_address === "string" ? lastEstimate.payment_address : "";
    if (addr) sendToAddress.value = addr;
  });

  function setStatus(message) {
    statusMessage.textContent = message || "";
  }

  function getTauriListen() {
    const t = window.__TAURI__;
    if (!t || typeof t !== "object") return null;
    const ev = t.event;
    if (ev && typeof ev.listen === "function") return ev.listen.bind(ev);
    return null;
  }

  try {
    const listen = getTauriListen();
    if (listen) {
      listen("kaspa_send_progress", (e) => {
        const p = e && e.payload ? e.payload : null;
        if (!p) return;
        const submitted = typeof p.submitted_chunks === "number" ? p.submitted_chunks : null;
        const total = typeof p.total_chunks === "number" ? p.total_chunks : null;
        if (typeof submitted === "number") setSendProgress(submitted, total);
      });
    }
  } catch {
    // ignore
  }

  function formatDuration(ms) {
    const totalSeconds = Math.max(0, Math.floor(ms / 1000));
    const h = Math.floor(totalSeconds / 3600);
    const m = Math.floor((totalSeconds % 3600) / 60);
    const s = totalSeconds % 60;
    const pad = (v) => String(v).padStart(2, "0");
    return h > 0 ? `${pad(h)}:${pad(m)}:${pad(s)}` : `${pad(m)}:${pad(s)}`;
  }

  function startSendTimer() {
    sendTimerStart = Date.now();
    if (sendElapsed) sendElapsed.textContent = "00:00";
    if (sendEta) sendEta.textContent = "";
    if (sendTimerInterval) clearInterval(sendTimerInterval);
    sendTimerInterval = setInterval(() => {
      if (!sendTimerStart) return;
      const ms = Date.now() - sendTimerStart;
      if (sendElapsed) sendElapsed.textContent = formatDuration(ms);
    }, 250);
  }

  function stopSendTimer() {
    if (sendTimerInterval) {
      clearInterval(sendTimerInterval);
      sendTimerInterval = null;
    }
    sendTimerStart = null;
  }

  function setSendProgress(submitted, total) {
    const done = typeof submitted === "number" && submitted >= 0 ? submitted : 0;
    const tot = typeof total === "number" && total > 0 ? total : null;

    if (sendProgressText) {
      sendProgressText.textContent = tot ? `${done}/${tot}` : `${done}`;
    }

    if (sendProgressBar) {
      if (tot) {
        sendProgressBar.max = tot;
        sendProgressBar.value = done;
      } else {
        sendProgressBar.max = 100;
        sendProgressBar.value = 0;
      }
    }

    if (sendProgressRingValue && sendProgressRingLabel) {
      const C = 326.7256;
      const pct = tot ? Math.max(0, Math.min(1, done / tot)) : 0;
      const pctText = `${Math.round(pct * 100)}%`;
      sendProgressRingLabel.textContent = pctText;
      sendProgressRingValue.style.strokeDashoffset = String(C * (1 - pct));
    }

    if (sendProgressRing) {
      sendProgressRing.classList.remove("pulse");
      void sendProgressRing.offsetWidth;
      sendProgressRing.classList.add("pulse");
    }

    if (tot) updateSendEta(done, tot);
  }

  function updateSendEta(submitted, total) {
    if (!sendEta) return;
    if (!sendTimerStart || !total || total <= 0 || typeof submitted !== "number" || submitted <= 0) {
      sendEta.textContent = "";
      return;
    }

    const elapsedSec = (Date.now() - sendTimerStart) / 1000;
    if (elapsedSec <= 0) {
      sendEta.textContent = "";
      return;
    }

    const rate = submitted / elapsedSec;
    if (!isFinite(rate) || rate <= 0) {
      sendEta.textContent = "";
      return;
    }
    const remaining = Math.max(0, total - submitted);
    const etaSec = remaining / rate;
    if (!isFinite(etaSec)) {
      sendEta.textContent = "";
      return;
    }

    sendEta.textContent = `· ETA ${formatDuration(etaSec * 1000)}`;
  }

  function startReceiveTimer() {
    receiveTimerStart = Date.now();
    if (receiveElapsed) receiveElapsed.textContent = "00:00";
    if (receiveTimerInterval) clearInterval(receiveTimerInterval);
    receiveTimerInterval = setInterval(() => {
      if (!receiveTimerStart) return;
      const ms = Date.now() - receiveTimerStart;
      if (receiveElapsed) receiveElapsed.textContent = formatDuration(ms);
    }, 250);
  }

  function stopReceiveTimer() {
    if (receiveTimerInterval) {
      clearInterval(receiveTimerInterval);
      receiveTimerInterval = null;
    }
    receiveTimerStart = null;
  }

  async function waitForReceivedFile(txId, startedAtMs) {
    const safeTxId = (txId || "").trim();
    if (!safeTxId) return;
    const targetName = `recv_${safeTxId}.bin`;
    const deadline = Date.now() + 10 * 60 * 1000;

    while (Date.now() < deadline) {
      try {
        const r = await fetch("/api/library", { cache: "no-store" });
        if (r.ok) {
          const data = await r.json();
          const files = data && Array.isArray(data.files) ? data.files : [];
          const f = files.find((x) => x && typeof x.name === "string" && x.name === targetName);
          const modified = f && typeof f.modified_unix_ms === "number" ? f.modified_unix_ms : null;
          if (f && (!modified || modified >= startedAtMs - 2000)) {
            const elapsedMs = receiveTimerStart ? Date.now() - receiveTimerStart : 0;
            stopReceiveTimer();
            const info = `Download completed in ${formatDuration(elapsedMs)}.`;
            if (receiveDownloadedInfo) receiveDownloadedInfo.textContent = info;
            setStatus(info);
            return;
          }
        }
      } catch {
        // ignore
      }

      await sleep(1000);
    }
  }

  function clearResults() {
    resultSection?.classList.add("hidden");
    if (resultFileName) resultFileName.textContent = "";
    if (resultFileSize) resultFileSize.textContent = "";
    if (resultChunkCount) resultChunkCount.textContent = "";
    if (resultTotalFee) resultTotalFee.textContent = "";
    if (resultEffectiveCost) resultEffectiveCost.textContent = "";
    if (resultExpectedAmount) resultExpectedAmount.textContent = "";
    if (resultPaymentAddress) resultPaymentAddress.textContent = "";
    if (resultJobId) resultJobId.textContent = "";
    if (resultStatus) resultStatus.textContent = "";
    lastEstimate = null;
  }

  function resetSession() {
    try {
      if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
      }
      currentJobId = null;

      stopSendTimer();
      stopReceiveTimer();

      selectedFile = null;
      if (fileInput) fileInput.value = "";
      if (dropZoneText) dropZoneText.textContent = "Drop a file here, or click to browse.";
      enableEstimate(false);
      clearResults();

      if (sendTxId) sendTxId.textContent = "";
      if (sendProgressText) sendProgressText.textContent = "";
      if (sendProgressBar) {
        sendProgressBar.value = 0;
        sendProgressBar.max = 100;
      }
      if (sendProgressRingLabel) sendProgressRingLabel.textContent = "0%";
      if (sendProgressRingValue) sendProgressRingValue.style.strokeDashoffset = "326.7256";
      if (sendElapsed) sendElapsed.textContent = "00:00";
      if (sendEta) sendEta.textContent = "";

      if (sendUsePaymentAddress) sendUsePaymentAddress.checked = false;
      if (sendToAddress) sendToAddress.value = "";
      if (sendAmountKas) sendAmountKas.value = "0";
      if (sendResumeFrom) sendResumeFrom.value = "";
      if (sendResumeOutputIndex) sendResumeOutputIndex.value = "1";

      if (receiveTxId) receiveTxId.value = "";
      if (receiveStartBlockHash) receiveStartBlockHash.value = "";
      if (receiveOutputName) receiveOutputName.value = "";
      if (receiveDownloadedInfo) receiveDownloadedInfo.textContent = "";
      if (receiveElapsed) receiveElapsed.textContent = "00:00";

      setStatus("");

      refreshLibrary();
    } catch (err) {
      console.error("resetSession error", err);
      setStatus(`Error: ${err instanceof Error ? err.message : String(err)}`);
    }
  }

  function enableEstimate(enabled) {
    estimateBtn.disabled = !enabled;
  }

  function onFileSelected(file) {
    selectedFile = file;
    if (file) {
      dropZoneText.textContent = `${file.name} (${formatBytes(file.size)})`;
      enableEstimate(true);
      clearResults();
      setStatus("");
    } else {
      dropZoneText.textContent = "Drop a file here, or click to browse.";
      enableEstimate(false);
    }
  }

  dropZone.addEventListener("click", () => {
    fileInput.click();
  });

  dropZone.addEventListener("dragover", (event) => {
    event.preventDefault();
    dropZone.classList.add("drag-over");
  });

  dropZone.addEventListener("dragleave", (event) => {
    event.preventDefault();
    dropZone.classList.remove("drag-over");
  });

  dropZone.addEventListener("drop", (event) => {
    event.preventDefault();
    dropZone.classList.remove("drag-over");
    const files = event.dataTransfer?.files;
    if (files && files.length > 0) {
      onFileSelected(files[0]);
      fileInput.value = "";
    }
  });

  fileInput.addEventListener("change", () => {
    if (fileInput.files && fileInput.files.length > 0) {
      onFileSelected(fileInput.files[0]);
    }
  });

  estimateBtn.addEventListener("click", async () => {
    if (!selectedFile) {
      setStatus("Please choose a file first.");
      return;
    }

    if (pollTimer) {
      clearInterval(pollTimer);
      pollTimer = null;
    }

    enableEstimate(false);
    setStatus("Uploading file and requesting estimate...");

    try {
      const formData = new FormData();
      const pk = (fromPrivateKey?.value || "").trim();
      const payer = (payerAddress?.value || "").trim();
      const amount = (storageAmountKas?.value || "0").trim();
      const rpc = (rpcUrl?.value || "").trim();

      if (payer.length > 0) formData.append("payer_address", payer);
      if (pk.length > 0) formData.append("from_private_key", pk);
      if (amount.length > 0) formData.append("amount", amount);
      if (rpc.length > 0) formData.append("rpc_url", rpc);
      formData.append("file", selectedFile);

      const response = await fetch("/api/estimate", {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || `Request failed with status ${response.status}`);
      }

      const data = await response.json();
      lastEstimate = data;
      populateResult(data);
      setStatus("Estimate received.");

      currentJobId = data.job_id || null;
      if (currentJobId) {
        if (pollTimer) clearInterval(pollTimer);
        pollTimer = setInterval(pollJobStatus, 5000);
      }
    } catch (err) {
      console.error(err);
      setStatus(`Error: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      enableEstimate(!!selectedFile);
    }
  });

  function populateResult(data) {
    resultSection.classList.remove("hidden");

    resultFileName.textContent = data.file_name || (selectedFile ? selectedFile.name : "");
    const sizeBytes = data.file_size_bytes ?? (selectedFile ? selectedFile.size : 0);
    resultFileSize.textContent = formatBytes(sizeBytes);

    if (typeof data.chunk_count !== "undefined") {
      resultChunkCount.textContent = String(data.chunk_count);
    }

    if (typeof data.total_network_fee_kas !== "undefined") {
      resultTotalFee.textContent = formatKas(data.total_network_fee_kas);
    }

    if (typeof data.effective_cost_per_mib_kas !== "undefined") {
      resultEffectiveCost.textContent = formatKas(data.effective_cost_per_mib_kas);
    }

    if (typeof data.expected_amount_kas !== "undefined") {
      resultExpectedAmount.textContent = formatKas(data.expected_amount_kas);
    }

    if (typeof data.payment_address === "string") {
      resultPaymentAddress.textContent = data.payment_address;
    }

    if (
      typeof data.payment_address === "string" &&
      sendToAddress &&
      sendUsePaymentAddress &&
      sendUsePaymentAddress.checked
    ) {
      sendToAddress.value = data.payment_address;
    }
    if (typeof data.expected_amount_kas === "number" && sendAmountKas) {
      sendAmountKas.value = String(data.expected_amount_kas);
    }

    if (typeof data.job_id === "string") {
      resultJobId.textContent = data.job_id;
    }

    if (typeof data.status === "string") {
      resultStatus.textContent = data.status;
    }
  }

  sendBtn?.addEventListener("click", async () => {
    if (!selectedFile) {
      setStatus("Please choose a file first.");
      return;
    }

    const pk = (fromPrivateKey?.value || "").trim();
    const to = (sendToAddress?.value || "").trim();
    const amount = (sendAmountKas?.value || "").trim();
    const rpc = (rpcUrl?.value || "").trim();
    const resumeFrom = (sendResumeFrom?.value || "").trim();
    const resumeOutputIndex = (sendResumeOutputIndex?.value || "1").trim();

    if (!pk) {
      setStatus("Send requires a private key.");
      return;
    }
    if (!to) {
      setStatus("Send requires a to address.");
      return;
    }
    if (!amount) {
      setStatus("Send requires an amount.");
      return;
    }

    try {
      sendBtn.disabled = true;
      setStatus("Queueing send job...");
      if (sendTxId) sendTxId.textContent = "";
      if (sendProgressText) sendProgressText.textContent = "";
      if (sendProgressBar) {
        sendProgressBar.value = 0;
        sendProgressBar.max = 100;
      }

      startSendTimer();

      const formData = new FormData();
      formData.append("from_private_key", pk);
      formData.append("to_address", to);
      formData.append("amount", amount);
      if (rpc) formData.append("rpc_url", rpc);
      if (resumeFrom) formData.append("resume_from", resumeFrom);
      if (resumeOutputIndex) formData.append("resume_output_index", resumeOutputIndex);
      formData.append("file", selectedFile);

      const response = await fetch("/api/send_async", {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || `Request failed with status ${response.status}`);
      }

      const data = await response.json();
      const jobId = data.job_id;
      if (!jobId) {
        throw new Error("send_async did not return a job_id");
      }
      setStatus(`Send job queued: ${jobId}. Waiting for completion...`);

      const poll = async () => {
        const r = await fetch(`/api/send_jobs/${jobId}`);
        if (!r.ok) return null;
        return await r.json();
      };

      const maxMs = 10 * 60 * 1000;
      const start = Date.now();

      while (Date.now() - start < maxMs) {
        const st = await poll();
        if (!st) {
          await sleep(500);
          continue;
        }

        if (typeof st.submitted_chunks === "number") {
          const total = typeof st.total_chunks === "number" ? st.total_chunks : null;
          setSendProgress(st.submitted_chunks, total);
        }

        if (typeof st.status === "string") {
          if (st.status === "succeeded") {
            if (sendTxId && typeof st.txid === "string") {
              sendTxId.textContent = st.txid;
            }
            if (receiveTxId && typeof st.txid === "string") {
              receiveTxId.value = st.txid;
            }

            if (sendProgressBar && typeof st.total_chunks === "number") {
              sendProgressBar.max = st.total_chunks;
              sendProgressBar.value = st.total_chunks;
            }
            if (typeof st.total_chunks === "number") setSendProgress(st.total_chunks, st.total_chunks);
            setStatus("Send succeeded.");
            stopSendTimer();
            return;
          }
          if (st.status === "failed" || st.status === "blocked") {
            const errText = typeof st.error === "string" ? st.error : "Send failed";
            const m = /already spent by transaction\s+([0-9a-fA-F]{64})\s+in the mempool/.exec(errText);
            if (m && sendResumeFrom) {
              sendResumeFrom.value = m[1];
              setStatus(
                `Send blocked by mempool (UTXO already spent). Wait for confirmation or try resume from = ${m[1]}.`
              );
            } else {
              setStatus(`Send ${st.status}: ${errText}`);
            }
            stopSendTimer();
            return;
          }
        }

        await sleep(500);
      }

      setStatus("Send job timed out waiting for completion.");
      stopSendTimer();
    } catch (err) {
      console.error(err);
      const msg = err instanceof Error ? err.message : String(err);
      const m = /already spent by transaction\s+([0-9a-fA-F]{64})\s+in the mempool/.exec(msg);
      if (m && sendResumeFrom) {
        sendResumeFrom.value = m[1];
        setStatus(
          `Kaspa rejected the send because funds are already being used by a tx still in the mempool. ` +
            `Wait for that tx to confirm, or try again with Resume from = ${m[1]}.`
        );
      } else {
        setStatus(`Error: ${msg}`);
      }
      stopSendTimer();
    } finally {
      sendBtn.disabled = false;
    }
  });

  function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  async function tauriSaveBytes(bytes, suggestedName) {
    const t = window.__TAURI__;
    const invoke = t && typeof t.invoke === "function" ? t.invoke.bind(t) : null;
    if (!invoke) {
      throw new Error("Tauri invoke is unavailable");
    }

    const fileDialog = t.dialog;
    const fs = t.fs;
    const path = t.path;
    if (!fileDialog || !fs || !path) {
      throw new Error("Tauri dialog/fs/path APIs are unavailable (check allowlist)");
    }

    const defaultDir = await path.downloadDir();
    const target = await fileDialog.save({
      defaultPath: `${defaultDir}${path.sep}${suggestedName}`,
    });
    if (!target) {
      throw new Error("Save cancelled");
    }

    await fs.writeBinaryFile({
      path: target,
      contents: bytes,
    });

    return target;
  }

  receiveBtn?.addEventListener("click", async () => {
    const txId = (receiveTxId?.value || "").trim();
    const rpc = (rpcUrl?.value || "").trim();
    const startBlockHash = (receiveStartBlockHash?.value || "").trim();
    const outputName = (receiveOutputName?.value || "").trim();

    if (!txId) {
      setStatus("Receive requires a transaction ID.");
      return;
    }

    try {
      receiveBtn.disabled = true;
      setStatus("Downloading received file...");
      if (receiveDownloadedInfo) receiveDownloadedInfo.textContent = "";

      startReceiveTimer();

      const invoke = getTauriInvoke();
      if (invoke || isDesktopTauri()) {
        const payload = {
          tx_id: txId,
          rpc_url: rpc || null,
          start_block_hash: startBlockHash || null,
          output_name: outputName || null,
        };

        const response = await fetch("/api/receive", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(payload),
        });

        if (!response.ok) {
          const errorText = await response.text();
          throw new Error(errorText || `Request failed with status ${response.status}`);
        }

        const contentDisposition = response.headers.get("content-disposition") || "";
        const m = /filename="([^"]+)"/.exec(contentDisposition);
        const serverName = m ? m[1] : "";
        const downloadName = outputName || serverName || "received.bin";

        const blob = await response.blob();
        const bytes = new Uint8Array(await blob.arrayBuffer());
        const savedTo = await tauriSaveBytes(bytes, downloadName);

        const info = `Saved ${formatBytes(bytes.length)} to ${savedTo}.`;
        if (receiveDownloadedInfo) receiveDownloadedInfo.textContent = info;
        setStatus(info);
        stopReceiveTimer();
        return;
      }

      const payload = {
        tx_id: txId,
        rpc_url: rpc || null,
        start_block_hash: startBlockHash || null,
        output_name: outputName || null,
      };

      const response = await fetch("/api/receive", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || `Request failed with status ${response.status}`);
      }

      const contentDisposition = response.headers.get("content-disposition") || "";
      const m = /filename="([^"]+)"/.exec(contentDisposition);
      const serverName = m ? m[1] : "";
      const downloadName = outputName || serverName || "received.bin";

      const blob = await response.blob();
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = downloadName;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);

      const info = `Received ${formatBytes(blob.size)}. Saving as ${downloadName}.`;
      if (receiveDownloadedInfo) receiveDownloadedInfo.textContent = info;
      setStatus(info);
      stopReceiveTimer();
    } catch (err) {
      console.error(err);
      setStatus(`Error: ${err instanceof Error ? err.message : String(err)}`);
      stopReceiveTimer();
    } finally {
      receiveBtn.disabled = false;
    }
  });

  async function pollJobStatus() {
    if (!currentJobId) return;

    try {
      const response = await fetch(`/api/jobs/${currentJobId}`);
      if (!response.ok) return;
      const data = await response.json();
      if (typeof data.status === "string") {
        resultStatus.textContent = data.status;
      }
    } catch (err) {
      console.error("pollJobStatus error", err);
    }
  }

  function formatBytes(bytes) {
    if (!bytes || bytes <= 0) return "0 B";
    const units = ["B", "KiB", "MiB", "GiB"];
    let value = bytes;
    let unitIndex = 0;
    while (value >= 1024 && unitIndex < units.length - 1) {
      value /= 1024;
      unitIndex += 1;
    }
    return `${value.toFixed(2)} ${units[unitIndex]}`;
  }

  function formatWhen(ms) {
    if (!ms || typeof ms !== "number") return "";
    try {
      return new Date(ms).toLocaleString();
    } catch {
      return "";
    }
  }

  function getTauriInvoke() {
    const t = window.__TAURI__;
    if (!t || typeof t !== "object") return null;
    if (typeof t.invoke === "function") return t.invoke.bind(t);
    const core = t.core;
    if (core && typeof core.invoke === "function") return core.invoke.bind(core);
    return null;
  }

  function isDesktopTauri() {
    try {
      if (getTauriInvoke()) return true;
      if (typeof window !== "undefined" && (window.__TAURI_IPC__ || window.__TAURI__)) return true;
      const ua = typeof navigator !== "undefined" ? navigator.userAgent : "";
      return /tauri/i.test(ua);
    } catch {
      return false;
    }
  }

  function clearLibraryList() {
    if (!libraryList) return;
    while (libraryList.firstChild) libraryList.removeChild(libraryList.firstChild);
  }

  async function downloadToDisk(url, filename) {
    const response = await fetch(url, { cache: "no-store" });
    if (!response.ok) {
      const t = await response.text().catch(() => "");
      throw new Error(t || `Request failed with status ${response.status}`);
    }

    const blob = await response.blob();
    const objectUrl = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = objectUrl;
    a.download = filename || "download.bin";
    document.body.appendChild(a);
    a.click();
    a.remove();
    URL.revokeObjectURL(objectUrl);
  }

  function renderLibrary(data) {
    if (!libraryList || !libraryEmpty || !libraryBaseDir) return;

    libraryBaseDir.textContent = data && typeof data.base_dir === "string" ? data.base_dir : "";

    const files = data && Array.isArray(data.files) ? data.files : [];
    clearLibraryList();

    if (files.length === 0) {
      libraryEmpty.classList.remove("hidden");
      return;
    }
    libraryEmpty.classList.add("hidden");

    const invoke = getTauriInvoke();

    for (const f of files) {
      const name = typeof f.name === "string" ? f.name : "";
      if (!name) continue;

      const item = document.createElement("div");
      item.className = "library-item";

      const meta = document.createElement("div");
      meta.className = "library-item__meta";

      const title = document.createElement("div");
      title.className = "library-item__name";
      title.textContent = name;

      const sub = document.createElement("div");
      sub.className = "library-item__sub";
      const sizeText = formatBytes(typeof f.size_bytes === "number" ? f.size_bytes : 0);
      const whenText = formatWhen(typeof f.modified_unix_ms === "number" ? f.modified_unix_ms : null);
      sub.textContent = whenText ? `${sizeText} · ${whenText}` : sizeText;

      meta.appendChild(title);
      meta.appendChild(sub);

      const actions = document.createElement("div");
      actions.className = "library-item__actions";

      const download = document.createElement("a");
      download.href = `/api/library/files/${encodeURIComponent(name)}`;
      download.textContent = "Download";
      download.setAttribute("download", name);
      if (invoke) {
        download.addEventListener("click", async (e) => {
          e.preventDefault();
          try {
            setStatus(`Downloading ${name}...`);
            await downloadToDisk(download.href, name);
            setStatus(`Downloaded ${name}.`);
          } catch (err) {
            console.error(err);
            setStatus(`Error: ${err instanceof Error ? err.message : String(err)}`);
          }
        });
      }
      actions.appendChild(download);

      const absPath = typeof f.path === "string" ? f.path : "";
      if (invoke && absPath) {
        const openBtn = document.createElement("button");
        openBtn.type = "button";
        openBtn.className = "btn-secondary";
        openBtn.textContent = "Open";
        openBtn.addEventListener("click", async () => {
          try {
            await invoke("open_file", { path: absPath });
          } catch (e) {
            console.error(e);
            setStatus(`Error: ${e instanceof Error ? e.message : String(e)}`);
          }
        });
        actions.appendChild(openBtn);

        const revealBtn = document.createElement("button");
        revealBtn.type = "button";
        revealBtn.className = "btn-secondary";
        revealBtn.textContent = "Show in folder";
        revealBtn.addEventListener("click", async () => {
          try {
            await invoke("reveal_in_folder", { path: absPath });
          } catch (e) {
            console.error(e);
            setStatus(`Error: ${e instanceof Error ? e.message : String(e)}`);
          }
        });
        actions.appendChild(revealBtn);
      }

      item.appendChild(meta);
      item.appendChild(actions);
      libraryList.appendChild(item);
    }
  }

  async function refreshLibrary() {
    if (!libraryList || !libraryEmpty || !libraryBaseDir) return;
    try {
      const r = await fetch("/api/library", { cache: "no-store" });
      if (!r.ok) {
        const t = await r.text();
        throw new Error(t || `Request failed with status ${r.status}`);
      }
      const data = await r.json();
      renderLibrary(data);
    } catch (e) {
      console.error(e);
      setStatus(`Error: ${e instanceof Error ? e.message : String(e)}`);
    }
  }

  libraryRefreshBtn?.addEventListener("click", refreshLibrary);
  refreshLibrary();

  function formatKas(value) {
    if (typeof value !== "number") return String(value);
    return value.toFixed(8);
  }
});
