async function init() {
  const loadingEl = document.getElementById('loading');
  const appEl = document.getElementById('app');

  try {
    // Import the WASM module
    const wasm = await import('./wasm/kavis_ui_story_web.js');
    await wasm.default();

    // Initialize the story gallery
    await wasm.run();

    // GPUI kendi canvas öğesini body içine ekler; yükleme kabuğu artık gerekli değil.
    if (appEl) {
      appEl.remove();
    }
  } catch (error) {
    console.error('Başlatılamadı:', error);

    // Show error message
    if (loadingEl) {
      loadingEl.innerHTML = `
        <div class="error">
          <h2>Uygulama yüklenemedi</h2>
          <p>${error.message || error}</p>
          <p style="margin-top: 10px; font-size: 14px;">
            Ayrıntılar için tarayıcı konsolunu kontrol edin.
          </p>
        </div>
      `;
    }
  }
}

init();
