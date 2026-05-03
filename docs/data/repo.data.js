const API_URL = "https://api.github.com/repos/hakantr/kavis-ui";
const FALLBACK = { stargazers_count: 0 };

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
  console.warn(`[repo.data] ${reason}, fallback kullanılıyor`);
  return FALLBACK;
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

    let data;
    try {
      data = await res.json();
    } catch (err) {
      return fallback(`GitHub API JSON yanıtı okunamadı: ${err.message}`);
    }

    if (typeof data.stargazers_count !== "number") {
      return fallback("GitHub API beklenen repo verisini döndürmedi");
    }

    return data;
  },
};
