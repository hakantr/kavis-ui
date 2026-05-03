const IGNORE_LOGINS = ["dependabot[bot]", "copilot"];
const API_URL =
  "https://api.github.com/repos/hakantr/kavis-ui/contributors";

function githubHeaders() {
  const headers = {
    Accept: "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
  };

  if (process.env.GITHUB_TOKEN) {
    headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  }

  return headers;
}

function fallback(reason) {
  console.warn(`[contributors.data] ${reason}, boş liste kullanılıyor`);
  return [];
}

export default {
  async load() {
    let res;
    try {
      res = await fetch(API_URL, { headers: githubHeaders() });
    } catch (err) {
      return fallback(`GitHub API isteği başarısız oldu: ${err.message}`);
    }

    if (!res.ok) {
      return fallback(`GitHub API ${res.status} ${res.statusText} döndü`);
    }

    let items;
    try {
      items = await res.json();
    } catch (err) {
      return fallback(`GitHub API JSON yanıtı okunamadı: ${err.message}`);
    }

    if (!Array.isArray(items)) {
      return fallback("GitHub API beklenen katkıcı listesini döndürmedi");
    }

    return items
      .filter((item) => !IGNORE_LOGINS.includes(item.login.toLowerCase()))
      .slice(0, 24);
  },
};
