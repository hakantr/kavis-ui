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
const zhSidebar = createSidebar("/zh-CN/docs/", "文档");

function createFooter(prefix = "", locale: "tr" | "zh" = "tr") {
  const contributorsText = locale === "zh" ? "贡献者" : "Katkıda Bulunanlar";
  const skillsText = locale === "zh" ? "技能" : "Yetenekler";
  const reportBugText = locale === "zh" ? "报告问题" : "Sorun Bildir";
  const discussionText = locale === "zh" ? "讨论" : "Tartışmalar";
  const galleryText = locale === "zh" ? "画廊" : "Galeri";
  const iconResourcesText =
    locale === "zh" ? "图标资源" : "Simge kaynakları";
  const message =
    locale === "zh"
      ? `Kavis UI 是一个基于 Apache-2.0 许可证的开源项目，
        由 <a href='https://longbridge.com' target='_blank'>Longbridge</a> 开发。`
      : `Kavis UI, Apache-2.0 lisansı altında yayımlanan açık kaynaklı bir projedir.`;

  return {
    message,
    copyright: `
      <a href="https://gpui.rs">GPUI</a>
      |
      <a href="/kavis-ui/gallery/" target="_blank">${galleryText}</a>
      |
      <a href="/kavis-ui${prefix}/contributors">${contributorsText}</a>
      |
      <a href="/kavis-ui${prefix}/skills" target="_blank">${skillsText}</a>
      |
      <a href="/kavis-ui/llms-full.txt" target="_blank">llms-full.txt</a>
      |
      <a href="https://github.com/hakantr/kavis-ui/issues" target="_blank">${reportBugText}</a>
      |
      <a href="https://github.com/hakantr/kavis-ui/discussions" target="_blank">${discussionText}</a>
      <br />
      ${iconResourcesText}: <a href="https://lucide.dev" target="_blank">Lucide</a>,
      <a href="https://isocons.app" target="_blank">Isocons</a>.
    `,
  };
}

function createNav(prefix = "", locale: "tr" | "zh" = "tr") {
  const homeText = locale === "zh" ? "首页" : "Ana Sayfa";
  const gettingStartedText = locale === "zh" ? "开始使用" : "Başlarken";
  const componentsText = locale === "zh" ? "组件" : "Bileşenler";
  const resourcesText = locale === "zh" ? "资源" : "Kaynaklar";
  const contributorsText = locale === "zh" ? "贡献者" : "Katkıda Bulunanlar";
  const releasesText = locale === "zh" ? "版本发布" : "Sürümler";
  const issuesText = locale === "zh" ? "问题" : "Sorunlar";
  const discussionText = locale === "zh" ? "讨论" : "Tartışmalar";
  const galleryText = locale === "zh" ? "画廊" : "Galeri";
  const apiDocText = locale === "zh" ? "API 文档" : "API Dokümanı";

  return [
    { text: homeText, link: `${prefix}/` || "/" },
    { text: gettingStartedText, link: `${prefix}/docs/getting-started` || "/docs/getting-started" },
    { text: componentsText, link: `${prefix}/docs/components` || "/docs/components" },
    { text: galleryText, link: "/gallery/", target: "_blank" },
    { text: apiDocText, link: "https://docs.rs/kavis-ui" },
    {
      text: resourcesText,
      items: [
        {
          text: contributorsText,
          link: `${prefix}/contributors` || "/contributors",
        },
        {
          text: releasesText,
          link: "https://github.com/hakantr/kavis-ui/releases",
        },
        {
          text: issuesText,
          link: "https://github.com/hakantr/kavis-ui/issues",
        },
        {
          text: discussionText,
          link: "https://github.com/hakantr/kavis-ui/discussions",
        },
      ],
    },
    {
      component: "LanguageSwitcher",
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
        nav: createNav("", "tr"),
        sidebar: trSidebar,
        footer: createFooter("", "tr"),
        editLink: {
          pattern:
            "https://github.com/hakantr/kavis-ui/edit/main/docs/:path",
          text: "Bu sayfayı düzenle",
        },
      },
    },
    "zh-CN": {
      label: "简体中文",
      lang: "zh-CN",
      link: "/zh-CN/",
      themeConfig: {
        ...sharedThemeConfig,
        nav: createNav("/zh-CN", "zh"),
        sidebar: zhSidebar,
        footer: createFooter("/zh-CN", "zh"),
        langMenuLabel: "语言",
        returnToTopLabel: "返回顶部",
        sidebarMenuLabel: "菜单",
        darkModeSwitchLabel: "外观",
        lightModeSwitchTitle: "切换到浅色模式",
        darkModeSwitchTitle: "切换到深色模式",
        editLink: {
          pattern:
            "https://github.com/hakantr/kavis-ui/edit/main/docs/:path",
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
