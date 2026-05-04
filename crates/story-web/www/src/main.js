function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function showError(title, message, detail) {
  const loadingEl = document.getElementById('loading');
  const appEl = document.getElementById('app');
  const markup = `
    <div class="error">
      <h2>${escapeHtml(title)}</h2>
      <p>${escapeHtml(message)}</p>
      ${detail ? `<pre>${escapeHtml(detail)}</pre>` : ''}
    </div>
  `;

  if (loadingEl) {
    loadingEl.innerHTML = markup;
    return;
  }

  const errorEl = document.createElement('div');
  errorEl.className = 'startup-error';
  errorEl.innerHTML = markup;

  if (appEl) {
    appEl.replaceWith(errorEl);
  } else {
    document.body.appendChild(errorEl);
  }
}

async function verifyWebGpu() {
  if (!window.isSecureContext) {
    throw new Error('WebGPU yalnızca güvenli bağlantılarda kullanılabilir. Sayfayı HTTPS üzerinden açın.');
  }

  if (!('gpu' in navigator)) {
    throw new Error('Bu tarayıcı WebGPU API desteği sunmuyor.');
  }

  let adapter;
  try {
    adapter = await navigator.gpu.requestAdapter({
      powerPreference: 'high-performance',
    });
  } catch (error) {
    throw new Error(`WebGPU adaptörü istenirken hata oluştu: ${error.message || error}`);
  }

  if (!adapter) {
    throw new Error(
      'WebGPU adaptörü bulunamadı. Donanım hızlandırmayı açın, tarayıcıyı güncelleyin veya Chrome/Edge gibi WebGPU destekli bir tarayıcı kullanın.',
    );
  }
}

async function init() {
  const appEl = document.getElementById('app');

  try {
    await verifyWebGpu();

    const wasm = await import('./wasm/kavis_ui_story_web.js');
    await wasm.default();

    await wasm.run();

    // GPUI kendi canvas öğesini body içine ekler; yükleme kabuğu artık gerekli değil.
    if (appEl) {
      appEl.remove();
    }
  } catch (error) {
    showError(
      'Galeri başlatılamadı',
      'Tarayıcı GPU bağlamı başlatılamadı.',
      error.message || error,
    );
  }
}

init();
