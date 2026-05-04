import { defineConfig } from "vitepress";
import type { UserConfig } from "vitepress";
import { generateSidebar } from "vitepress-sidebar";
import llmstxt from "vitepress-plugin-llms";
import tailwindcss from "@tailwindcss/vite";
import { lightTheme, darkTheme } from "./language";
import { ViteToml } from "vite-plugin-toml";

/**
 * https://github.com/jooy2/vitepress-sidebar
 */
function createSidebar(scanStartPath: string, rootGroupText: string) {
  return generateSidebar([
    {
      scanStartPath,
      rootGroupText,
      collapsed: false,
      useTitleFromFrontmatter: true,
      useTitleFromFileHeading: true,
      sortMenusByFrontmatterOrder: true,
      includeRootIndexFile: false,
    },
  ]) as any;
}

const trSidebar = createSidebar("/docs/", "Giriş");

function createFooter(prefix = "") {
  const message = `Kavis UI, Apache-2.0 lisansı altında yayımlanan açık kaynaklı bir projedir.`;

  return {
    message,
    copyright: `
      <a href="https://gpui.rs">GPUI</a>
      |
      <a href="/kavis-ui/gallery/" target="_blank">Galeri</a>
      |
      <a href="/kavis-ui${prefix}/contributors">Katkıda Bulunanlar</a>
      |
      <a href="/kavis-ui${prefix}/skills" target="_blank">Yetenekler</a>
      |
      <a href="/kavis-ui/llms-full.txt" target="_blank">llms-full.txt</a>
      |
      <a href="https://github.com/hakantr/kavis-ui/issues" target="_blank">Sorun Bildir</a>
      |
      <a href="https://github.com/hakantr/kavis-ui/discussions" target="_blank">Tartışmalar</a>
      <br />
      Simge kaynakları: <a href="https://lucide.dev" target="_blank">Lucide</a>,
      <a href="https://isocons.app" target="_blank">Isocons</a>.
    `,
  };
}

function createNav(prefix = "") {
  return [
    { text: "Ana Sayfa", link: `${prefix}/` || "/" },
    { text: "Başlarken", link: `${prefix}/docs/getting-started` || "/docs/getting-started" },
    { text: "Bileşenler", link: `${prefix}/docs/components` || "/docs/components" },
    { text: "Galeri", link: "/gallery/", target: "_blank" },
    {
      text: "Kaynaklar",
      items: [
        {
          text: "Katkıda Bulunanlar",
          link: `${prefix}/contributors` || "/contributors",
        },
        {
          text: "Sürümler",
          link: "https://github.com/hakantr/kavis-ui/releases",
        },
        {
          text: "Sorunlar",
          link: "https://github.com/hakantr/kavis-ui/issues",
        },
        {
          text: "Tartışmalar",
          link: "https://github.com/hakantr/kavis-ui/discussions",
        },
      ],
    },
    {
      component: "GitHubStar",
    },
  ];
}

const sharedThemeConfig = {
  logo: {
    light: "/logo.svg",
    dark: "/logo-dark.svg",
  },
  socialLinks: null,
  search: {
    provider: "local",
  },
};

// https://vitepress.dev/reference/site-config
const config: UserConfig = {
  title: "Kavis UI",
  base: "/kavis-ui/",
  description:
    "GPUI kullanarak çapraz platform masaüstü uygulamaları geliştirmek için Rust GUI bileşenleri.",
  cleanUrls: true,
  head: [
    [
      "link",
      {
        rel: "icon",
        href: "/kavis-ui/logo.svg",
        media: "(prefers-color-scheme: light)",
      },
    ],
    [
      "link",
      {
        rel: "icon",
        href: "/kavis-ui/logo-dark.svg",
        media: "(prefers-color-scheme: dark)",
      },
    ],
  ],
  vite: {
    plugins: [llmstxt(), tailwindcss(), ViteToml()],
  },
  themeConfig: sharedThemeConfig,
  locales: {
    root: {
      label: "Türkçe",
      lang: "tr-TR",
      themeConfig: {
        ...sharedThemeConfig,
        langMenuLabel: "Diller",
        returnToTopLabel: "Yukarı dön",
        sidebarMenuLabel: "Menü",
        darkModeSwitchLabel: "Görünüm",
        lightModeSwitchTitle: "Açık moda geç",
        darkModeSwitchTitle: "Koyu moda geç",
        outline: {
          label: "Bu Sayfada",
        },
        docFooter: {
          prev: "Önceki",
          next: "Sonraki",
        },
        nav: createNav(),
        sidebar: trSidebar,
        footer: createFooter(),
        editLink: {
          pattern:
            "https://github.com/hakantr/kavis-ui/edit/main/docs/:path",
          text: "Bu sayfayı düzenle",
        },
      },
    },
  },
  markdown: {
    math: true,
    defaultHighlightLang: "rs",
    theme: {
      light: lightTheme,
      dark: darkTheme,
    },
  },
};

export default defineConfig(config);
