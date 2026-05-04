use std::ops::Range;

use crate::ham_gpui::{
    App, Context, Div, InteractiveElement as _, IntoElement, ParentElement as _, Pixels,
    SharedString, Stateful, Styled as _, Window, div,
};

use crate::{
    BilesenBoyutu, EtkinTema as _, Simge, SimgeAdi, h_flex,
    menu::AcilirMenu,
    table::{Column, ColumnGroup, ColumnSort, TabloDurumu, loading::Loading},
};

/// Bir temsilci özellik için providing veri ve çizim için bir tablo.
#[allow(unused)]
pub trait TabloTemsilcisi: Sized + 'static {
    /// sayı sütunlar içinde tablo döndürür.
    fn columns_count(&self, cx: &App) -> usize;

    /// sayı satırlar içinde tablo döndürür.
    fn rows_count(&self, cx: &App) -> usize;

    /// Verilen indeksteki tablo sütununu döndürür.
    ///
    /// Bu yalnızca çağırır üzerinde Tablo prepare veya refresh.
    fn column(&self, col_ix: usize, cx: &App) -> Column;

    /// Verilen indeksteki sütunda sıralama yapar.
    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: ColumnSort,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) {
    }

    /// tablo başlık satır. çizer.
    fn render_header(
        &mut self,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> Stateful<Div> {
        div().id("header")
    }

    /// grup headers tanımlar (olabilir multi-seviye) döndürür.
    ///
    /// Varsayılan olarak None döndürür; bu, grup başlığı olmadığı anlamına gelir.
    fn group_headers(&self, cx: &App) -> Option<Vec<Vec<ColumnGroup>>> {
        None
    }

    /// özel çizer için bir grup başlık hücre.
    /// Receives grup etiket, mantıksal col_span, ve piksel genişlik.
    fn render_group_th(
        &mut self,
        label: &SharedString,
        _col_span: usize,
        width: Pixels,
        _window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement {
        div()
            .w(width)
            .h_full()
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .border_r_1()
            .border_color(cx.theme().border)
            .child(label.clone())
    }

    /// Verilen sütun indeksindeki başlık hücresini çizer; varsayılan olarak sütun adını kullanır.
    fn render_th(
        &mut self,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .child(self.column(col_ix, cx).name.clone())
    }

    /// Verilen satır ve sütundaki satırı çizer.
    ///
    /// Değil dahil eder tablo başlık satır.
    fn render_tr(
        &mut self,
        row_ix: usize,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> Stateful<Div> {
        div().id(("row", row_ix))
    }

    /// Verilen satır indeksi için bağlam menüsünü çizer.
    fn baglam_menusu(
        &mut self,
        row_ix: usize,
        menu: AcilirMenu,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> AcilirMenu {
        menu
    }

    /// Verilen satır ve sütundaki hücreyi çizer.
    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement;

    /// `col_ix` konumundaki sütunu `to_ix` konumundaki sütundan önceye taşır.
    fn move_column(
        &mut self,
        col_ix: usize,
        to_ix: usize,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) {
    }

    /// Tablo boşken gösterilecek öğeyi döndürür.
    fn render_empty(
        &mut self,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement {
        h_flex()
            .size_full()
            .justify_center()
            .text_color(cx.theme().muted_foreground.opacity(0.6))
            .child(Simge::new(SimgeAdi::Inbox).size_12())
            .into_any_element()
    }

    /// true göstermek için Yükleme görünümü. döndürür.
    fn loading(&self, cx: &App) -> bool {
        false
    }

    /// Tablo yüklenirken gösterilecek öğeyi döndürür; varsayılan yerleşik iskelet yükleme görünümüdür.
    ///
    /// Boyut, tablonun boyutudur.
    fn render_loading(
        &mut self,
        size: BilesenBoyutu,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement {
        Loading::new().size(size)
    }

    /// Alta kaydırıldığında daha fazla veri yüklemeyi etkinleştirmek için true döndürür.
    ///
    /// varsayılan: false
    fn has_more(&self, cx: &App) -> bool {
        false
    }

    /// bir eşik değer (n satırlar), course, olduğunda kaydırma için alt, döndürür.
    /// remaining sayı satırlar triggers `load_more`.
    /// Bu değer ilk yüklenen toplam satır sayısından küçük olmalıdır.
    ///
    /// varsayılan: 20 satırlar
    fn load_more_threshold(&self) -> usize {
        20
    }

    /// Tablo alta kaydırıldığında daha fazla veri yükler.
    ///
    /// Bu işlem bir arka plan görevinde yürütülür.
    ///
    /// Tablo alta yaklaştığında bu her zaman çağrılır,
    /// bu yüzden yüklenecek başka veri olup olmadığını kontrol etmeli veya yükleme durumunu kilitlemelisiniz.
    fn load_more(&mut self, window: &mut Window, cx: &mut Context<TabloDurumu<Self>>) {}

    /// son boş sütun, varsayılan için boş. çizer.
    fn render_last_empty_col(
        &mut self,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement {
        h_flex().w_3().h_full().flex_shrink_0()
    }

    /// Görünür satır aralığı değiştiğinde çağrılır.
    ///
    /// NOT: Bu yöntemin hızlı olduğundan emin olun; sık çağrılır.
    ///
    /// Bazı veri güncellemelerini işlemek ve yalnızca görünür satırları güncellemek için kullanılabilir.
    /// Verinin arka plan görevinde güncellendiğinden emin olun.
    fn visible_rows_changed(
        &mut self,
        visible_range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) {
    }

    /// Görünür sütun aralığı değiştiğinde çağrılır.
    ///
    /// NOT: Bu yöntemin hızlı olduğundan emin olun; sık çağrılır.
    ///
    /// Bazı veri güncellemelerini işlemek ve yalnızca görünür satırları güncellemek için kullanılabilir.
    /// Verinin arka plan görevinde güncellendiğinden emin olun.
    fn visible_columns_changed(
        &mut self,
        visible_range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<TabloDurumu<Self>>,
    ) {
    }

    /// metin representation bir hücre için export purposes (e.g., CSV export) döndürür.
    ///
    /// bir boş metin ile varsayılan. Implement bu yöntem için destek export. döndürür.
    /// metin olmalıdır formatted olarak onu olmalı appear içinde exported veri.
    fn cell_text(&self, row_ix: usize, col_ix: usize, cx: &App) -> String {
        String::new()
    }
}
