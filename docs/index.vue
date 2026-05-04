<template>
    <div class="banner">
        <h1>Kavis UI</h1>
        <div class="banner-description">
            {{ bannerPrefix }}
            <a href="https://gpui.rs" target="_blank">GPUI</a>
            {{ bannerSuffix }}
        </div>
        <div class="actions">
            <a :href="gettingStartedHref" class="btn-primary">{{ getStartedText }}</a>
            <a :href="componentsHref"><Blocks /> {{ componentsText }}</a>
        </div>
        <div class="version">
            {{ versionLabel }}
            <a href="https://crates.io/crates/kavis-ui" target="_blank">{{
                VERSION
            }}</a>
        </div>
    </div>
    <div class="features">
        <div class="feature-card">
            <h3>
                <div class="icon bg-green-500 dark:bg-green-700">
                    <Blocks />
                </div>
                <div>{{ features.componentCount.title }}</div>
            </h3>
            <div>
                {{ features.componentCount.description }}
            </div>
        </div>
        <div class="feature-card">
            <h3>
                <div class="icon bg-blue-500 dark:bg-blue-700">
                    <Zap />
                </div>
                <div>{{ features.performance.title }}</div>
            </h3>
            <div>
                {{ features.performance.description }}
            </div>
        </div>

        <div class="feature-card">
            <h3>
                <div class="icon bg-red-500 dark:bg-red-700">
                    <Palette />
                </div>
                <div>{{ features.theme.title }}</div>
            </h3>
            <div>
                {{ features.theme.description }}
            </div>
        </div>

        <div class="feature-card">
            <h3>
                <div class="icon bg-yellow-500 dark:bg-yellow-700">
                    <Layout />
                </div>
                <div>{{ features.layout.title }}</div>
            </h3>
            <div>
                {{ features.layout.description }}
            </div>
        </div>

        <div class="feature-card">
            <h3>
                <div class="icon bg-pink-500 dark:bg-pink-700">
                    <BarChart3 />
                </div>
                <div>{{ features.chart.title }}</div>
            </h3>
            <div>
                {{ features.chart.description }}
            </div>
        </div>

        <div class="feature-card">
            <h3>
                <div class="icon bg-cyan-500 dark:bg-cyan-700">
                    <SquareCode />
                </div>
                <div>{{ features.editor.title }}</div>
            </h3>
            <div>
                {{ features.editor.description }}
            </div>
        </div>
    </div>
</template>

<script setup>
import { computed } from "vue";
import { withBase } from "vitepress";
import {
    Blocks,
    Zap,
    Palette,
    Layout,
    BarChart3,
    SquareCode,
} from "lucide-vue-next";

const gettingStartedHref = computed(
    () => withBase("/docs/getting-started"),
);
const componentsHref = computed(() =>
    withBase("/docs/components"),
);
const bannerPrefix = "Rust + ";
const bannerSuffix = " ile güçlü çapraz platform masaüstü uygulamaları geliştirin.";
const getStartedText = "Başlarken";
const componentsText = "Bileşenler";
const versionLabel = "Sürüm:";
const features = {
    componentCount: {
        title: "60+ Bileşen",
        description:
            "Zengin özellikli uygulamalar kurmak için kapsamlı çapraz platform masaüstü UI bileşenleri.",
    },
    performance: {
        title: "Yüksek Performans",
        description:
            "Büyük veri kümelerini düşük bellek kullanımıyla akıcı çizmek için sanallaştırılmış tablo ve liste bileşenleri.",
    },
    theme: {
        title: "Temalanabilir",
        description:
            "Yerleşik tema sistemi, 20+ tema ve hazır açık/koyu mod desteği.",
    },
    layout: {
        title: "Esnek Düzenler",
        description:
            "Panel yerleşimleri, yeniden boyutlandırılabilir paneller ve farklı uygulama yapıları için serbest düzenler.",
    },
    chart: {
        title: "Veri Görselleştirme",
        description:
            "Çizgi, bar, alan ve pasta grafiklerle veri göstermek için yerleşik grafik bileşenleri.",
    },
    editor: {
        title: "Kod Editörü",
        description:
            "Tree-sitter ve Rope tabanlı, LSP ve sözdizimi vurgulama destekli yüksek performanslı kod editörü.",
    },
};
</script>

<style lang="scss">
@reference "./.vitepress/theme/style.css";

.banner {
    @apply flex flex-col gap-2 lg:gap-4 -mt-20  py-12 xl:py-30 text-center border border-b-0 border-(--border);

    background: url("/home.svg") no-repeat;
    background-position: bottom -90px right -90px;

    h1 {
        @apply mt-20 text-3xl xl:text-5xl font-bold mb-2 text-(--primary);
    }
    .banner-description {
        @apply text-lg xl:text-2xl text-(--muted-foreground);
    }
    .actions {
        @apply gap-4 flex justify-center text-sm;
        a {
            @apply flex items-center h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5 no-underline
            bg-(--secondary) hover:bg-(--secondary)/70 text-(--secondary-foreground);

            &.btn-primary {
                @apply bg-(--primary) hover:bg-(--primary)/90 text-(--primary-foreground);
            }

            .lucide {
                @apply w-4 h-4;
            }
        }
    }
    .version {
        @apply text-sm text-(--muted-foreground) pb-10;
        a {
            @apply text-(--muted-foreground) no-underline hover:underline;
        }
    }
}

.features {
    @apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 mb-12 border-b border-r  border-(--border);
}

.feature-card {
    @apply flex flex-col text-sm gap-2 py-3.5 px-5 border border-b-0 border-(--border);
    @apply border-r-0 last:border-r-0 md:last:border-b-0 lg:last:border-b-0;

    h3 {
        @apply m-0 p-0 text-lg text-(--primary) flex gap-3 items-center;

        .icon {
            @apply flex h-9 w-9 items-center justify-center rounded-md text-white;

            .lucide {
                @apply w-5 h-5;
            }
        }
    }
}
</style>
