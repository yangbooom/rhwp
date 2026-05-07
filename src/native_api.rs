//! 외부(특히 Tauri 데스크탑 앱) 컨슈머를 위한 공개 facade.
//!
//! 이 모듈은 `pub(crate)` 으로 정의된 내부 API들 중 외부 어댑터 (현재 fhwp)에서
//! 실제로 필요한 부분만을 얇은 래퍼로 노출한다. 목적은 다음과 같다:
//!
//! - `pub(crate)` 캡슐화는 그대로 유지 (내부 리팩토링 자유도 확보)
//! - 외부 사용자에게 "쓸 만한 표면"을 명시적으로 제공
//! - `_native` 접미사 같은 내부 명명 규칙을 외부에 노출시키지 않음
//!
//! 외부 컨슈머는 보통 다음 형태로 사용한다:
//! ```ignore
//! use rhwp::DocumentCore;
//! let mut doc = DocumentCore::from_bytes(&bytes)?;
//! let json = doc.get_caret_position()?;
//! ```

use crate::DocumentCore;
use crate::error::HwpError;
use crate::model::document::{DocInfo, Document, Section};
use crate::model::paragraph::Paragraph;
use crate::model::style::{CharShapeMods, ParaShapeMods};
use crate::renderer::style_resolver::{ResolvedStyleSet, resolve_styles};

// ─── 내부 타입 재공개 ─────────────────────────────────────────────────
//
// 외부 컨슈머가 nav-result 등을 다룰 때 필요한 타입을 type alias 로 노출한다.
// (모듈 자체는 비공개로 유지하면서 타입만 외부 경로 `rhwp::native_api::X` 로 사용 가능.)

pub type NavContextEntry = crate::document_core::queries::doc_tree_nav::NavContextEntry;
pub type NavResult = crate::document_core::queries::doc_tree_nav::NavResult;
pub type OverflowLink = crate::document_core::queries::doc_tree_nav::OverflowLink;

// ─── 헬퍼 함수 래퍼 ───────────────────────────────────────────────────
//
// `document_core::helpers::*` 의 `pub(crate)` 헬퍼들을 동일 시그니처의 `pub fn`
// 으로 래핑해 재노출한다 (다른 모듈에서 정의된 `pub(crate)` 항목은 그대로 `pub use`
// 할 수 없기 때문).

/// 문단 내 컨트롤 위치(텍스트 오프셋) 목록.
#[inline]
pub fn find_control_text_positions(para: &Paragraph) -> Vec<usize> {
    crate::document_core::helpers::find_control_text_positions(para)
}

/// JSON 문자열 이스케이프.
#[inline]
pub fn json_escape(s: &str) -> String {
    crate::document_core::helpers::json_escape(s)
}

/// JSON 객체에서 i32 값 추출.
#[inline]
pub fn json_i32(json: &str, key: &str) -> Option<i32> {
    crate::document_core::helpers::json_i32(json, key)
}

/// JSON 객체에서 문자열 값 추출.
#[inline]
pub fn json_str(json: &str, key: &str) -> Option<String> {
    crate::document_core::helpers::json_str(json, key)
}

/// 문단의 논리적 길이(텍스트+컨트롤).
#[inline]
pub fn logical_paragraph_length(para: &Paragraph) -> usize {
    crate::document_core::helpers::logical_paragraph_length(para)
}

/// 논리 오프셋 → 텍스트 오프셋 변환.
#[inline]
pub fn logical_to_text_offset(para: &Paragraph, logical_offset: usize) -> (usize, bool) {
    crate::document_core::helpers::logical_to_text_offset(para, logical_offset)
}

/// 텍스트 오프셋 → 논리 오프셋 변환.
#[inline]
pub fn text_to_logical_offset(para: &Paragraph, text_offset: usize) -> usize {
    crate::document_core::helpers::text_to_logical_offset(para, text_offset)
}

/// 캐릭터 모양 변경 JSON 파서.
#[inline]
pub fn parse_char_shape_mods(json: &str) -> CharShapeMods {
    crate::document_core::helpers::parse_char_shape_mods(json)
}

/// 문단 모양 변경 JSON 파서.
#[inline]
pub fn parse_para_shape_mods(json: &str) -> ParaShapeMods {
    crate::document_core::helpers::parse_para_shape_mods(json)
}

/// 사설 영역(PUA)에 매핑된 글머리 기호 문자를 표시용 문자로 변환.
#[inline]
pub fn map_pua_bullet_char(ch: char) -> char {
    crate::renderer::layout::map_pua_bullet_char(ch)
}

// ─── 자유 함수 ────────────────────────────────────────────────────────

/// 문서 정보로부터 ResolvedStyleSet 을 다시 계산한다.
///
/// `DocumentCore::recompute_styles` 를 사용하면 이 함수를 직접 호출할 필요가 없다.
#[inline]
pub fn resolve_styles_for(doc_info: &DocInfo, dpi: f64) -> ResolvedStyleSet {
    resolve_styles(doc_info, dpi)
}

// ─── DocumentCore 필드 액세서 / 설정자 ────────────────────────────────

impl DocumentCore {
    // -- 읽기 전용 액세서 --
    //
    // (기본 `document()` 은 commands/document.rs 에 이미 정의되어 있음.)

    /// 내부 `Document` IR 에 대한 가변 참조.
    ///
    /// 외부 어댑터에서 모델 구조 (`sections`, `doc_info` 등) 를 직접 변경해야 할 때 사용.
    #[inline]
    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    /// 현재 DPI.
    #[inline]
    pub fn dpi(&self) -> f64 {
        self.dpi
    }

    /// 대체 폰트 경로.
    #[inline]
    pub fn fallback_font(&self) -> &str {
        &self.fallback_font
    }

    /// 원본 파일 형식 (HWP / HWPX).
    #[inline]
    pub fn source_format(&self) -> crate::parser::FileFormat {
        self.source_format
    }

    /// 문단 부호(¶) 표시 여부.
    #[inline]
    pub fn show_paragraph_marks(&self) -> bool {
        self.show_paragraph_marks
    }

    /// 조판 부호 표시 여부.
    #[inline]
    pub fn show_control_codes(&self) -> bool {
        self.show_control_codes
    }

    /// 투명 테두리 표시 여부.
    #[inline]
    pub fn show_transparent_borders(&self) -> bool {
        self.show_transparent_borders
    }

    /// 클리핑 적용 여부.
    #[inline]
    pub fn clip_enabled(&self) -> bool {
        self.clip_enabled
    }

    /// 디버그 오버레이 표시 여부.
    #[inline]
    pub fn debug_overlay(&self) -> bool {
        self.debug_overlay
    }

    // -- 설정자 (캐시 무효화 포함) --

    /// 문단 부호 표시 토글. 페이지 트리 캐시를 무효화한다.
    #[inline]
    pub fn set_show_paragraph_marks(&mut self, enabled: bool) {
        self.show_paragraph_marks = enabled;
        self.invalidate_page_tree_cache();
    }

    /// 조판 부호 표시 토글. 페이지 트리 캐시를 무효화한다.
    #[inline]
    pub fn set_show_control_codes(&mut self, enabled: bool) {
        self.show_control_codes = enabled;
        self.invalidate_page_tree_cache();
    }

    /// 투명 테두리 표시 토글. 페이지 트리 캐시를 무효화한다.
    #[inline]
    pub fn set_show_transparent_borders(&mut self, enabled: bool) {
        self.show_transparent_borders = enabled;
        self.invalidate_page_tree_cache();
    }

    /// 클리핑 적용 토글. 페이지 트리 캐시를 무효화한다.
    #[inline]
    pub fn set_clip_enabled(&mut self, enabled: bool) {
        self.clip_enabled = enabled;
        self.invalidate_page_tree_cache();
    }

    /// 디버그 오버레이 토글.
    #[inline]
    pub fn set_debug_overlay(&mut self, enabled: bool) {
        self.debug_overlay = enabled;
    }

    /// 파일 이름 (머리말/꼬리말 필드 치환에 사용).
    #[inline]
    pub fn set_file_name(&mut self, name: String) {
        self.file_name = name;
    }

    /// 대체 폰트 경로.
    #[inline]
    pub fn set_fallback_font(&mut self, path: String) {
        self.fallback_font = path;
    }

    /// `doc_info` 가 변경된 뒤 스타일 세트를 현재 DPI 로 재계산한다.
    ///
    /// 이전에 외부에서 `doc.styles = resolve_styles(&doc.document.doc_info, doc.dpi)`
    /// 처럼 직접 대입하던 패턴을 대체한다.
    #[inline]
    pub fn recompute_styles(&mut self) {
        self.styles = resolve_styles(&self.document.doc_info, self.dpi);
    }

    // -- 페이지 트리 캐시 무효화 (외부 노출) --

    /// 페이지 렌더 트리 캐시를 무효화한다.
    ///
    /// 외부에서 `document_mut()` 등을 통해 모델을 직접 변경한 후 다음 렌더링에서
    /// 변경 사항이 반영되도록 호출한다.
    #[inline]
    pub fn invalidate_render_cache(&self) {
        self.invalidate_page_tree_cache();
    }

    // -- 비-_native pub(crate) 메서드 래퍼 --

    /// 지정 구역을 다시 빌드한다 (페이지네이션 포함).
    #[inline]
    pub fn rebuild_section_at(&mut self, section_idx: usize) {
        self.rebuild_section(section_idx);
    }

    /// 페이지 렌더 트리를 빌드한다 (캐시됨). HTML/JSON 직렬화 등에서 사용.
    #[inline]
    pub fn page_render_tree(
        &self,
        page_num: u32,
    ) -> Result<crate::renderer::render_tree::PageRenderTree, HwpError> {
        self.build_page_tree_cached(page_num)
    }

    /// 표 셀의 bbox 들을 페이지 단위로 추출 (JSON).
    #[inline]
    pub fn table_cell_bboxes_from_page(
        &self,
        section_idx: usize,
        parent_para_idx: usize,
        control_idx: usize,
        page_hint: usize,
    ) -> Result<String, HwpError> {
        self.get_table_cell_bboxes_from_page(section_idx, parent_para_idx, control_idx, page_hint)
    }

    /// 셀 경로상의 문단 참조 (없으면 None).
    #[inline]
    pub fn cell_paragraph_ref(
        &self,
        section_idx: usize,
        parent_para_idx: usize,
        control_idx: usize,
        cell_idx: usize,
        cell_para_idx: usize,
    ) -> Option<&Paragraph> {
        self.get_cell_paragraph_ref(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    /// 경로 기반 컨테이너의 문단 수.
    #[inline]
    pub fn container_para_count_by_path(
        &self,
        sec: usize,
        parent_para: usize,
        path: &[(usize, usize, usize)],
    ) -> Result<usize, HwpError> {
        self.resolve_container_para_count_by_path(sec, parent_para, path)
    }

    /// 경로 기반 단일 문단 참조.
    #[inline]
    pub fn paragraph_by_path(
        &self,
        sec: usize,
        parent_para: usize,
        path: &[(usize, usize, usize)],
    ) -> Result<&Paragraph, HwpError> {
        self.resolve_paragraph_by_path(sec, parent_para, path)
    }

    /// 글자 모양 ID로부터 char 속성 JSON 문자열 빌드.
    #[inline]
    pub fn char_properties_json_by_id(&self, char_shape_id: u16) -> String {
        self.build_char_properties_json_by_id(char_shape_id)
    }

    /// 문단 모양 ID로부터 para 속성 JSON 문자열 빌드.
    #[inline]
    pub fn para_properties_json(&self, para_shape_id: u16, sec_idx: usize) -> String {
        self.build_para_properties_json(para_shape_id, sec_idx)
    }

    /// `last_rendered_para_in_container` 의 외부 공개 별칭.
    #[inline]
    pub fn last_rendered_para_in_container_at(
        &self,
        section_idx: usize,
        parent_para: usize,
        ctrl_idx: usize,
        cell_idx: usize,
    ) -> Option<usize> {
        self.last_rendered_para_in_container(section_idx, parent_para, ctrl_idx, cell_idx)
    }

    /// 구역 내 글상자 오버플로우 링크 목록.
    ///
    /// 외부에서 `navigate_next_editable_at` 호출 시 인자로 사용한다.
    /// 반환 타입은 내부 타입을 그대로 노출한다.
    #[inline]
    pub fn overflow_links(
        &self,
        section_idx: usize,
    ) -> Vec<crate::document_core::queries::doc_tree_nav::OverflowLink> {
        self.get_overflow_links(section_idx)
    }

    /// 다음 편집 가능 위치를 탐색한다 (DFS 트리 순회).
    ///
    /// `parse_nav_context` / `fix_context_text_positions` 와 함께 사용한다.
    #[allow(clippy::too_many_arguments)]
    pub fn navigate_next_editable_at(
        &self,
        section_idx: usize,
        para_idx: usize,
        char_offset: usize,
        delta: i32,
        context: &[NavContextEntry],
        max_para: Option<usize>,
        overflow_links: &[crate::document_core::queries::doc_tree_nav::OverflowLink],
    ) -> NavResult {
        self.navigate_next_editable(
            section_idx,
            para_idx,
            char_offset,
            delta,
            context,
            max_para,
            overflow_links,
        )
    }

    // -- 정적 헬퍼 (DocumentCore::X) --

    /// JSON 문자열로부터 `(parent_para_idx, ctrl, cell)` 경로를 파싱한다.
    #[inline]
    pub fn parse_cell_path_json(path_json: &str) -> Result<Vec<(usize, usize, usize)>, HwpError> {
        Self::parse_cell_path(path_json)
    }

    /// 외부 어댑터에서 보낸 nav-context JSON 파싱.
    #[inline]
    pub fn parse_nav_context_json(json: &str) -> Vec<NavContextEntry> {
        Self::parse_nav_context(json)
    }

    /// `NavResult` 를 JSON 문자열로 직렬화.
    #[inline]
    pub fn nav_result_to_json_string(result: &NavResult) -> String {
        Self::nav_result_to_json(result)
    }

    /// 컨텍스트의 `ctrl_text_pos` 를 현재 모델 상태에 맞게 보정한다.
    pub fn fix_nav_context_positions(
        sections: &[crate::model::document::Section],
        sec: usize,
        context: &[NavContextEntry],
    ) -> Vec<NavContextEntry> {
        Self::fix_context_text_positions(sections, sec, context)
    }

    /// 문단의 텍스트 길이 (논리 단위) 헬퍼.
    #[inline]
    pub fn logical_paragraph_len(para: &Paragraph) -> usize {
        logical_paragraph_length(para)
    }

    /// 논리 → 텍스트 오프셋 변환 헬퍼.
    #[inline]
    pub fn logical_to_text(para: &Paragraph, logical_offset: usize) -> (usize, bool) {
        logical_to_text_offset(para, logical_offset)
    }

    /// 텍스트 → 논리 오프셋 변환 헬퍼.
    #[inline]
    pub fn text_to_logical(para: &Paragraph, text_offset: usize) -> usize {
        text_to_logical_offset(para, text_offset)
    }
}

// AUTO-GENERATED method wrappers (in impl DocumentCore in native_api.rs)
impl DocumentCore {
    #[inline]
    pub fn add_bookmark(&mut self, sec: usize, para: usize, char_offset: usize, name: &str) -> Result<String, HwpError> {
        self.add_bookmark_native(sec, para, char_offset, name)
    }
    #[inline]
    pub fn apply_cell_style(&mut self, sec_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, style_id: usize) -> Result<String, HwpError> {
        self.apply_cell_style_native(sec_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, style_id)
    }
    #[inline]
    pub fn apply_char_format_in_cell(&mut self, sec_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, start_offset: usize, end_offset: usize, props_json: &str) -> Result<String, HwpError> {
        self.apply_char_format_in_cell_native(sec_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, start_offset, end_offset, props_json)
    }
    #[inline]
    pub fn apply_char_format(&mut self, sec_idx: usize, para_idx: usize, start_offset: usize, end_offset: usize, props_json: &str) -> Result<String, HwpError> {
        self.apply_char_format_native(sec_idx, para_idx, start_offset, end_offset, props_json)
    }
    #[inline]
    pub fn apply_hf_template(&mut self, section_idx: usize, is_header: bool, apply_to: u8, template_id: u8) -> Result<String, HwpError> {
        self.apply_hf_template_native(section_idx, is_header, apply_to, template_id)
    }
    #[inline]
    pub fn apply_para_format_in_cell(&mut self, sec_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, props_json: &str) -> Result<String, HwpError> {
        self.apply_para_format_in_cell_native(sec_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, props_json)
    }
    #[inline]
    pub fn apply_para_format_in_hf(&mut self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize, props_json: &str) -> Result<String, HwpError> {
        self.apply_para_format_in_hf_native(section_idx, is_header, apply_to, hf_para_idx, props_json)
    }
    #[inline]
    pub fn apply_para_format(&mut self, sec_idx: usize, para_idx: usize, props_json: &str) -> Result<String, HwpError> {
        self.apply_para_format_native(sec_idx, para_idx, props_json)
    }
    #[inline]
    pub fn apply_style(&mut self, sec_idx: usize, para_idx: usize, style_id: usize) -> Result<String, HwpError> {
        self.apply_style_native(sec_idx, para_idx, style_id)
    }
    #[inline]
    pub fn begin_batch(&mut self) -> Result<String, HwpError> {
        self.begin_batch_native()
    }
    #[inline]
    pub fn change_shape_z_order(&mut self, section_idx: usize, para_idx: usize, control_idx: usize, operation: &str) -> Result<String, HwpError> {
        self.change_shape_z_order_native(section_idx, para_idx, control_idx, operation)
    }
    #[inline]
    pub fn clear_clipboard(&mut self) {
        self.clear_clipboard_native()
    }
    #[inline]
    pub fn clipboard_has_control(&self) -> bool {
        self.clipboard_has_control_native()
    }
    #[inline]
    pub fn convert_to_editable(&mut self) -> Result<String, HwpError> {
        self.convert_to_editable_native()
    }
    #[inline]
    pub fn copy_control(&mut self, section_idx: usize, para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.copy_control_native(section_idx, para_idx, control_idx)
    }
    #[inline]
    pub fn copy_selection_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, start_cell_para_idx: usize, start_char_offset: usize, end_cell_para_idx: usize, end_char_offset: usize) -> Result<String, HwpError> {
        self.copy_selection_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset)
    }
    #[inline]
    pub fn copy_selection(&mut self, section_idx: usize, start_para_idx: usize, start_char_offset: usize, end_para_idx: usize, end_char_offset: usize) -> Result<String, HwpError> {
        self.copy_selection_native(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset)
    }
    #[inline]
    pub fn create_blank_document(&mut self) -> Result<String, HwpError> {
        self.create_blank_document_native()
    }
    #[inline]
    pub fn create_header_footer(&mut self, section_idx: usize, is_header: bool, apply_to: u8) -> Result<String, HwpError> {
        self.create_header_footer_native(section_idx, is_header, apply_to)
    }
    #[inline]
    pub fn create_shape_control(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, width: u32, height: u32, horz_offset: u32, vert_offset: u32, treat_as_char: bool, text_wrap_str: &str, shape_type: &str, line_flip_x: bool, line_flip_y: bool, polygon_points: &[crate::model::Point]) -> Result<String, HwpError> {
        self.create_shape_control_native(section_idx, para_idx, char_offset, width, height, horz_offset, vert_offset, treat_as_char, text_wrap_str, shape_type, line_flip_x, line_flip_y, polygon_points)
    }
    #[inline]
    pub fn create_table_ex(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, row_count: u16, col_count: u16, treat_as_char: bool, col_widths_hu: Option<&[u32]>) -> Result<String, HwpError> {
        self.create_table_ex_native(section_idx, para_idx, char_offset, row_count, col_count, treat_as_char, col_widths_hu)
    }
    #[inline]
    pub fn create_table(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, row_count: u16, col_count: u16) -> Result<String, HwpError> {
        self.create_table_native(section_idx, para_idx, char_offset, row_count, col_count)
    }
    #[inline]
    pub fn delete_bookmark(&mut self, sec: usize, para: usize, ctrl_idx: usize) -> Result<String, HwpError> {
        self.delete_bookmark_native(sec, para, ctrl_idx)
    }
    #[inline]
    pub fn delete_header_footer(&mut self, section_idx: usize, is_header: bool, apply_to: u8) -> Result<String, HwpError> {
        self.delete_header_footer_native(section_idx, is_header, apply_to)
    }
    #[inline]
    pub fn delete_picture_control(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.delete_picture_control_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn delete_range(&mut self, section_idx: usize, start_para: usize, start_offset: usize, end_para: usize, end_offset: usize, cell_ctx: Option<(usize, usize, usize)>) -> Result<String, HwpError> {
        self.delete_range_native(section_idx, start_para, start_offset, end_para, end_offset, cell_ctx)
    }
    #[inline]
    pub fn delete_shape_control(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.delete_shape_control_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn delete_table_column(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, col_idx: u16) -> Result<String, HwpError> {
        self.delete_table_column_native(section_idx, parent_para_idx, control_idx, col_idx)
    }
    #[inline]
    pub fn delete_table_control(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.delete_table_control_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn delete_table_row(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, row_idx: u16) -> Result<String, HwpError> {
        self.delete_table_row_native(section_idx, parent_para_idx, control_idx, row_idx)
    }
    #[inline]
    pub fn delete_text_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize, count: usize) -> Result<String, HwpError> {
        self.delete_text_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, count)
    }
    #[inline]
    pub fn delete_text_in_footnote(&mut self, section_idx: usize, para_idx: usize, control_idx: usize, fn_para_idx: usize, char_offset: usize, count: usize) -> Result<String, HwpError> {
        self.delete_text_in_footnote_native(section_idx, para_idx, control_idx, fn_para_idx, char_offset, count)
    }
    #[inline]
    pub fn delete_text_in_header_footer(&mut self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize, char_offset: usize, count: usize) -> Result<String, HwpError> {
        self.delete_text_in_header_footer_native(section_idx, is_header, apply_to, hf_para_idx, char_offset, count)
    }
    #[inline]
    pub fn delete_text(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, count: usize) -> Result<String, HwpError> {
        self.delete_text_native(section_idx, para_idx, char_offset, count)
    }
    #[inline]
    pub fn discard_snapshot(&mut self, id: u32) {
        self.discard_snapshot_native(id)
    }
    #[inline]
    pub fn end_batch(&mut self) -> Result<String, HwpError> {
        self.end_batch_native()
    }
    #[inline]
    pub fn export_control_html(&self, section_idx: usize, para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.export_control_html_native(section_idx, para_idx, control_idx)
    }
    #[inline]
    pub fn export_hwp(&self) -> Result<Vec<u8>, HwpError> {
        self.export_hwp_native()
    }
    #[inline]
    pub fn export_hwpx(&self) -> Result<Vec<u8>, HwpError> {
        self.export_hwpx_native()
    }
    #[inline]
    pub fn export_selection_html(&self, section_idx: usize, start_para_idx: usize, start_char_offset: usize, end_para_idx: usize, end_char_offset: usize) -> Result<String, HwpError> {
        self.export_selection_html_native(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset)
    }
    #[inline]
    pub fn export_selection_in_cell_html(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, start_cell_para_idx: usize, start_char_offset: usize, end_cell_para_idx: usize, end_char_offset: usize) -> Result<String, HwpError> {
        self.export_selection_in_cell_html_native(section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset)
    }
    #[inline]
    pub fn find_nearest_control_backward(&self, section_idx: usize, para_idx: usize, char_offset: usize) -> String {
        self.find_nearest_control_backward_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn find_nearest_control_forward(&self, section_idx: usize, para_idx: usize, char_offset: usize) -> String {
        self.find_nearest_control_forward_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn find_next_editable_control(&self, section_idx: usize, para_idx: usize, ctrl_idx: i32, delta: i32) -> String {
        self.find_next_editable_control_native(section_idx, para_idx, ctrl_idx, delta)
    }
    #[inline]
    pub fn find_or_create_font_id(&mut self, name: &str) -> i32 {
        self.find_or_create_font_id_native(name)
    }
    #[inline]
    pub fn get_bookmarks(&self) -> Result<String, HwpError> {
        self.get_bookmarks_native()
    }
    #[inline]
    pub fn get_caret_position(&self) -> Result<String, HwpError> {
        self.get_caret_position_native()
    }
    #[inline]
    pub fn get_cell_char_properties_at(&self, sec_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_cell_char_properties_at_native(sec_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset)
    }
    #[inline]
    pub fn get_cell_info_by_path(&self, section_idx: usize, parent_para_idx: usize, path_json: &str) -> Result<String, HwpError> {
        self.get_cell_info_by_path_native(section_idx, parent_para_idx, path_json)
    }
    #[inline]
    pub fn get_cell_info(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize) -> Result<String, HwpError> {
        self.get_cell_info_native(section_idx, parent_para_idx, control_idx, cell_idx)
    }
    #[inline]
    pub fn get_cell_para_properties_at(&self, sec_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize) -> Result<String, HwpError> {
        self.get_cell_para_properties_at_native(sec_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx)
    }
    #[inline]
    pub fn get_cell_paragraph_count(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize) -> Result<usize, HwpError> {
        self.get_cell_paragraph_count_native(section_idx, parent_para_idx, control_idx, cell_idx)
    }
    #[inline]
    pub fn get_cell_paragraph_length(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize) -> Result<usize, HwpError> {
        self.get_cell_paragraph_length_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx)
    }
    #[inline]
    pub fn get_cell_properties(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize) -> Result<String, HwpError> {
        self.get_cell_properties_native(section_idx, parent_para_idx, control_idx, cell_idx)
    }
    #[inline]
    pub fn get_char_properties_at(&self, sec_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_char_properties_at_native(sec_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn get_clipboard_text(&self) -> String {
        self.get_clipboard_text_native()
    }
    #[inline]
    pub fn get_control_image_data(&self, section_idx: usize, para_idx: usize, control_idx: usize) -> Result<Vec<u8>, HwpError> {
        self.get_control_image_data_native(section_idx, para_idx, control_idx)
    }
    #[inline]
    pub fn get_control_image_mime(&self, section_idx: usize, para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_control_image_mime_native(section_idx, para_idx, control_idx)
    }
    #[inline]
    pub fn get_cursor_rect_by_path(&self, section_idx: usize, parent_para_idx: usize, path_json: &str, char_offset: usize) -> Result<String, HwpError> {
        self.get_cursor_rect_by_path_native(section_idx, parent_para_idx, path_json, char_offset)
    }
    #[inline]
    pub fn get_cursor_rect_in_cell(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_cursor_rect_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset)
    }
    #[inline]
    pub fn get_cursor_rect_in_footnote(&self, page_num: u32, footnote_index: usize, fn_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_cursor_rect_in_footnote_native(page_num, footnote_index, fn_para_idx, char_offset)
    }
    #[inline]
    pub fn get_cursor_rect_in_header_footer(&self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize, char_offset: usize, preferred_page: i32) -> Result<String, HwpError> {
        self.get_cursor_rect_in_header_footer_native(section_idx, is_header, apply_to, hf_para_idx, char_offset, preferred_page)
    }
    #[inline]
    pub fn get_cursor_rect(&self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_cursor_rect_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn get_equation_properties(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: Option<usize>, cell_para_idx: Option<usize>) -> Result<String, HwpError> {
        self.get_equation_properties_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx)
    }
    #[inline]
    pub fn get_footnote_info(&self, section_idx: usize, para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_footnote_info_native(section_idx, para_idx, control_idx)
    }
    #[inline]
    pub fn get_form_object_at(&self, page_num: u32, x: f64, y: f64) -> Result<String, crate::error::HwpError> {
        self.get_form_object_at_native(page_num, x, y)
    }
    #[inline]
    pub fn get_form_object_info(&self, sec: usize, para: usize, ci: usize) -> Result<String, crate::error::HwpError> {
        self.get_form_object_info_native(sec, para, ci)
    }
    #[inline]
    pub fn get_form_value(&self, sec: usize, para: usize, ci: usize) -> Result<String, crate::error::HwpError> {
        self.get_form_value_native(sec, para, ci)
    }
    #[inline]
    pub fn get_header_footer_list(&self, current_section_idx: usize, current_is_header: bool, current_apply_to: u8) -> Result<String, HwpError> {
        self.get_header_footer_list_native(current_section_idx, current_is_header, current_apply_to)
    }
    #[inline]
    pub fn get_header_footer(&self, section_idx: usize, is_header: bool, apply_to: u8) -> Result<String, HwpError> {
        self.get_header_footer_native(section_idx, is_header, apply_to)
    }
    #[inline]
    pub fn get_header_footer_para_info(&self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize) -> Result<String, HwpError> {
        self.get_header_footer_para_info_native(section_idx, is_header, apply_to, hf_para_idx)
    }
    #[inline]
    pub fn get_line_info_in_cell(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_line_info_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset)
    }
    #[inline]
    pub fn get_line_info(&self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.get_line_info_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn get_page_control_layout(&self, page_num: u32) -> Result<String, HwpError> {
        self.get_page_control_layout_native(page_num)
    }
    #[inline]
    pub fn get_page_def(&self, section_idx: usize) -> Result<String, HwpError> {
        self.get_page_def_native(section_idx)
    }
    #[inline]
    pub fn get_page_footnote_info(&self, page_num: u32, footnote_index: usize) -> Result<String, HwpError> {
        self.get_page_footnote_info_native(page_num, footnote_index)
    }
    #[inline]
    pub fn get_page_hide(&self, section_idx: usize, para_idx: usize) -> Result<String, crate::error::HwpError> {
        self.get_page_hide_native(section_idx, para_idx)
    }
    #[inline]
    pub fn get_page_info(&self, page_num: u32) -> Result<String, HwpError> {
        self.get_page_info_native(page_num)
    }
    #[inline]
    pub fn get_page_of_position(&self, section_idx: usize, para_idx: usize) -> Result<String, HwpError> {
        self.get_page_of_position_native(section_idx, para_idx)
    }
    #[inline]
    pub fn get_page_text_layout(&self, page_num: u32) -> Result<String, HwpError> {
        self.get_page_text_layout_native(page_num)
    }
    #[inline]
    pub fn get_para_properties_at(&self, sec_idx: usize, para_idx: usize) -> Result<String, HwpError> {
        self.get_para_properties_at_native(sec_idx, para_idx)
    }
    #[inline]
    pub fn get_para_properties_in_hf(&self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize) -> Result<String, HwpError> {
        self.get_para_properties_in_hf_native(section_idx, is_header, apply_to, hf_para_idx)
    }
    #[inline]
    pub fn get_paragraph_count(&self, section_idx: usize) -> Result<usize, HwpError> {
        self.get_paragraph_count_native(section_idx)
    }
    #[inline]
    pub fn get_paragraph_length(&self, section_idx: usize, para_idx: usize) -> Result<usize, HwpError> {
        self.get_paragraph_length_native(section_idx, para_idx)
    }
    #[inline]
    pub fn get_picture_properties(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_picture_properties_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn get_position_of_page(&self, global_page: usize) -> Result<String, HwpError> {
        self.get_position_of_page_native(global_page)
    }
    #[inline]
    pub fn get_section_def(&self, section_idx: usize) -> Result<String, HwpError> {
        self.get_section_def_native(section_idx)
    }
    #[inline]
    pub fn get_selection_rects(&self, section_idx: usize, start_para_idx: usize, start_char_offset: usize, end_para_idx: usize, end_char_offset: usize, cell_ctx: Option<(usize, usize, usize)>) -> Result<String, HwpError> {
        self.get_selection_rects_native(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset, cell_ctx)
    }
    #[inline]
    pub fn get_shape_properties(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_shape_properties_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn get_table_bbox(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_table_bbox_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn get_table_cell_bboxes_by_path(&self, section_idx: usize, parent_para_idx: usize, path_json: &str) -> Result<String, HwpError> {
        self.get_table_cell_bboxes_by_path_native(section_idx, parent_para_idx, path_json)
    }
    #[inline]
    pub fn get_table_cell_bboxes(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_table_cell_bboxes_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn get_table_dimensions_by_path(&self, section_idx: usize, parent_para_idx: usize, path_json: &str) -> Result<String, HwpError> {
        self.get_table_dimensions_by_path_native(section_idx, parent_para_idx, path_json)
    }
    #[inline]
    pub fn get_table_dimensions(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_table_dimensions_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn get_table_properties(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.get_table_properties_native(section_idx, parent_para_idx, control_idx)
    }
    #[inline]
    pub fn get_text_in_cell(&self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize, count: usize) -> Result<String, HwpError> {
        self.get_text_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, count)
    }
    #[inline]
    pub fn get_text_range(&self, section_idx: usize, para_idx: usize, char_offset: usize, count: usize) -> Result<String, HwpError> {
        self.get_text_range_native(section_idx, para_idx, char_offset, count)
    }
    #[inline]
    pub fn get_textbox_control_index(&self, section_idx: usize, para_idx: usize) -> i32 {
        self.get_textbox_control_index_native(section_idx, para_idx)
    }
    #[inline]
    pub fn group_shapes(&mut self, section_idx: usize, targets: &[(usize, usize)]) -> Result<String, HwpError> {
        self.group_shapes_native(section_idx, targets)
    }
    #[inline]
    pub fn has_internal_clipboard(&self) -> bool {
        self.has_internal_clipboard_native()
    }
    #[inline]
    pub fn hit_test_footnote(&self, page_num: u32, x: f64, y: f64) -> Result<String, HwpError> {
        self.hit_test_footnote_native(page_num, x, y)
    }
    #[inline]
    pub fn hit_test_header_footer(&self, page_num: u32, x: f64, y: f64) -> Result<String, HwpError> {
        self.hit_test_header_footer_native(page_num, x, y)
    }
    #[inline]
    pub fn hit_test_in_footnote(&self, page_num: u32, x: f64, y: f64) -> Result<String, HwpError> {
        self.hit_test_in_footnote_native(page_num, x, y)
    }
    #[inline]
    pub fn hit_test_in_header_footer(&self, page_num: u32, is_header: bool, x: f64, y: f64) -> Result<String, HwpError> {
        self.hit_test_in_header_footer_native(page_num, is_header, x, y)
    }
    #[inline]
    pub fn hit_test(&self, page_num: u32, x: f64, y: f64) -> Result<String, HwpError> {
        self.hit_test_native(page_num, x, y)
    }
    #[inline]
    pub fn insert_column_break(&mut self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.insert_column_break_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn insert_field_in_hf(&mut self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize, char_offset: usize, field_type: u8) -> Result<String, HwpError> {
        self.insert_field_in_hf_native(section_idx, is_header, apply_to, hf_para_idx, char_offset, field_type)
    }
    #[inline]
    pub fn insert_footnote(&mut self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.insert_footnote_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn insert_page_break(&mut self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.insert_page_break_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn insert_picture(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, image_data: &[u8], width: u32, height: u32, natural_width_px: u32, natural_height_px: u32, extension: &str, description: &str) -> Result<String, HwpError> {
        self.insert_picture_native(section_idx, para_idx, char_offset, image_data, width, height, natural_width_px, natural_height_px, extension, description)
    }
    #[inline]
    pub fn insert_table_column(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, col_idx: u16, right: bool) -> Result<String, HwpError> {
        self.insert_table_column_native(section_idx, parent_para_idx, control_idx, col_idx, right)
    }
    #[inline]
    pub fn insert_table_row(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, row_idx: u16, below: bool) -> Result<String, HwpError> {
        self.insert_table_row_native(section_idx, parent_para_idx, control_idx, row_idx, below)
    }
    #[inline]
    pub fn insert_text_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize, text: &str) -> Result<String, HwpError> {
        self.insert_text_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, text)
    }
    #[inline]
    pub fn insert_text_in_footnote(&mut self, section_idx: usize, para_idx: usize, control_idx: usize, fn_para_idx: usize, char_offset: usize, text: &str) -> Result<String, HwpError> {
        self.insert_text_in_footnote_native(section_idx, para_idx, control_idx, fn_para_idx, char_offset, text)
    }
    #[inline]
    pub fn insert_text_in_header_footer(&mut self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize, char_offset: usize, text: &str) -> Result<String, HwpError> {
        self.insert_text_in_header_footer_native(section_idx, is_header, apply_to, hf_para_idx, char_offset, text)
    }
    #[inline]
    pub fn insert_text(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, text: &str) -> Result<String, HwpError> {
        self.insert_text_native(section_idx, para_idx, char_offset, text)
    }
    #[inline]
    pub fn measure_width_diagnostic(&self, section_idx: usize, para_idx: usize) -> Result<String, HwpError> {
        self.measure_width_diagnostic_native(section_idx, para_idx)
    }
    #[inline]
    pub fn merge_paragraph_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize) -> Result<String, HwpError> {
        self.merge_paragraph_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx)
    }
    #[inline]
    pub fn merge_paragraph_in_footnote(&mut self, section_idx: usize, para_idx: usize, control_idx: usize, fn_para_idx: usize) -> Result<String, HwpError> {
        self.merge_paragraph_in_footnote_native(section_idx, para_idx, control_idx, fn_para_idx)
    }
    #[inline]
    pub fn merge_paragraph_in_header_footer(&mut self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize) -> Result<String, HwpError> {
        self.merge_paragraph_in_header_footer_native(section_idx, is_header, apply_to, hf_para_idx)
    }
    #[inline]
    pub fn merge_paragraph(&mut self, section_idx: usize, para_idx: usize) -> Result<String, HwpError> {
        self.merge_paragraph_native(section_idx, para_idx)
    }
    #[inline]
    pub fn merge_table_cells(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, start_row: u16, start_col: u16, end_row: u16, end_col: u16) -> Result<String, HwpError> {
        self.merge_table_cells_native(section_idx, parent_para_idx, control_idx, start_row, start_col, end_row, end_col)
    }
    #[inline]
    pub fn move_line_endpoint(&mut self, section_idx: usize, para_idx: usize, control_idx: usize, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Result<String, HwpError> {
        self.move_line_endpoint_native(section_idx, para_idx, control_idx, start_x, start_y, end_x, end_y)
    }
    #[inline]
    pub fn move_table_offset(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, delta_h: i32, delta_v: i32) -> Result<String, HwpError> {
        self.move_table_offset_native(section_idx, parent_para_idx, control_idx, delta_h, delta_v)
    }
    #[inline]
    pub fn move_vertical_by_path(&self, section_idx: usize, parent_para_idx: usize, path_json: &str, char_offset: usize, delta: i32, preferred_x: f64) -> Result<String, HwpError> {
        self.move_vertical_by_path_native(section_idx, parent_para_idx, path_json, char_offset, delta, preferred_x)
    }
    #[inline]
    pub fn move_vertical(&self, sec: usize, para: usize, char_offset: usize, delta: i32, preferred_x: f64, cell_ctx: Option<(usize, usize, usize, usize)>) -> Result<String, HwpError> {
        self.move_vertical_native(sec, para, char_offset, delta, preferred_x, cell_ctx)
    }
    #[inline]
    pub fn navigate_header_footer_by_page(&self, current_page: u32, is_header: bool, direction: i32) -> Result<String, HwpError> {
        self.navigate_header_footer_by_page_native(current_page, is_header, direction)
    }
    #[inline]
    pub fn paste_control(&mut self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.paste_control_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn paste_html_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize, html: &str) -> Result<String, HwpError> {
        self.paste_html_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, html)
    }
    #[inline]
    pub fn paste_html(&mut self, section_idx: usize, para_idx: usize, char_offset: usize, html: &str) -> Result<String, HwpError> {
        self.paste_html_native(section_idx, para_idx, char_offset, html)
    }
    #[inline]
    pub fn paste_internal_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.paste_internal_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset)
    }
    #[inline]
    pub fn paste_internal(&mut self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.paste_internal_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn rename_bookmark(&mut self, sec: usize, para: usize, ctrl_idx: usize, new_name: &str) -> Result<String, HwpError> {
        self.rename_bookmark_native(sec, para, ctrl_idx, new_name)
    }
    #[inline]
    pub fn render_equation_preview(&self, script: &str, font_size_hwpunit: u32, color: u32) -> Result<String, HwpError> {
        self.render_equation_preview_native(script, font_size_hwpunit, color)
    }
    #[inline]
    pub fn render_page_canvas(&self, page_num: u32) -> Result<u32, HwpError> {
        self.render_page_canvas_native(page_num)
    }
    #[inline]
    pub fn render_page_html(&self, page_num: u32) -> Result<String, HwpError> {
        self.render_page_html_native(page_num)
    }
    #[inline]
    pub fn render_page_svg(&self, page_num: u32) -> Result<String, HwpError> {
        self.render_page_svg_native(page_num)
    }
    #[inline]
    pub fn replace_all(&mut self, query: &str, new_text: &str, case_sensitive: bool) -> Result<String, HwpError> {
        self.replace_all_native(query, new_text, case_sensitive)
    }
    #[inline]
    pub fn replace_text(&mut self, sec: usize, para: usize, char_offset: usize, length: usize, new_text: &str) -> Result<String, HwpError> {
        self.replace_text_native(sec, para, char_offset, length, new_text)
    }
    #[inline]
    pub fn resize_table_cells(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, json: &str) -> Result<String, HwpError> {
        self.resize_table_cells_native(section_idx, parent_para_idx, control_idx, json)
    }
    #[inline]
    pub fn restore_snapshot(&mut self, id: u32) -> Result<String, HwpError> {
        self.restore_snapshot_native(id)
    }
    #[inline]
    pub fn save_snapshot(&mut self) -> u32 {
        self.save_snapshot_native()
    }
    #[inline]
    pub fn search_text(&self, query: &str, from_sec: usize, from_para: usize, from_char: usize, forward: bool, case_sensitive: bool) -> Result<String, HwpError> {
        self.search_text_native(query, from_sec, from_para, from_char, forward, case_sensitive)
    }
    #[inline]
    pub fn set_cell_properties(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, json: &str) -> Result<String, HwpError> {
        self.set_cell_properties_native(section_idx, parent_para_idx, control_idx, cell_idx, json)
    }
    #[inline]
    pub fn set_column_def(&mut self, section_idx: usize, column_count: u16, column_type: u8, same_width: bool, spacing_hu: i16) -> Result<String, HwpError> {
        self.set_column_def_native(section_idx, column_count, column_type, same_width, spacing_hu)
    }
    #[inline]
    pub fn set_equation_properties(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: Option<usize>, cell_para_idx: Option<usize>, props_json: &str) -> Result<String, HwpError> {
        self.set_equation_properties_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, props_json)
    }
    #[inline]
    pub fn set_form_value_in_cell(&mut self, sec: usize, table_para: usize, table_ci: usize, cell_idx: usize, cell_para: usize, form_ci: usize, value_json: &str) -> Result<String, crate::error::HwpError> {
        self.set_form_value_in_cell_native(sec, table_para, table_ci, cell_idx, cell_para, form_ci, value_json)
    }
    #[inline]
    pub fn set_form_value(&mut self, sec: usize, para: usize, ci: usize, value_json: &str) -> Result<String, crate::error::HwpError> {
        self.set_form_value_native(sec, para, ci, value_json)
    }
    #[inline]
    pub fn set_numbering_restart(&mut self, section_idx: usize, para_idx: usize, mode: u8, start_num: u32) -> Result<String, crate::error::HwpError> {
        self.set_numbering_restart_native(section_idx, para_idx, mode, start_num)
    }
    #[inline]
    pub fn set_page_def(&mut self, section_idx: usize, json: &str) -> Result<String, HwpError> {
        self.set_page_def_native(section_idx, json)
    }
    #[inline]
    pub fn set_page_hide(&mut self, section_idx: usize, para_idx: usize, hide_header: bool, hide_footer: bool, hide_master_page: bool, hide_border: bool, hide_fill: bool, hide_page_num: bool) -> Result<String, crate::error::HwpError> {
        self.set_page_hide_native(section_idx, para_idx, hide_header, hide_footer, hide_master_page, hide_border, hide_fill, hide_page_num)
    }
    #[inline]
    pub fn set_picture_properties(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, props_json: &str) -> Result<String, HwpError> {
        self.set_picture_properties_native(section_idx, parent_para_idx, control_idx, props_json)
    }
    #[inline]
    pub fn set_section_def_all(&mut self, json: &str) -> Result<String, HwpError> {
        self.set_section_def_all_native(json)
    }
    #[inline]
    pub fn set_section_def(&mut self, section_idx: usize, json: &str) -> Result<String, HwpError> {
        self.set_section_def_native(section_idx, json)
    }
    #[inline]
    pub fn set_shape_properties(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, props_json: &str) -> Result<String, HwpError> {
        self.set_shape_properties_native(section_idx, parent_para_idx, control_idx, props_json)
    }
    #[inline]
    pub fn set_table_properties(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, json: &str) -> Result<String, HwpError> {
        self.set_table_properties_native(section_idx, parent_para_idx, control_idx, json)
    }
    #[inline]
    pub fn split_paragraph_in_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, cell_idx: usize, cell_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.split_paragraph_in_cell_native(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset)
    }
    #[inline]
    pub fn split_paragraph_in_footnote(&mut self, section_idx: usize, para_idx: usize, control_idx: usize, fn_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.split_paragraph_in_footnote_native(section_idx, para_idx, control_idx, fn_para_idx, char_offset)
    }
    #[inline]
    pub fn split_paragraph_in_header_footer(&mut self, section_idx: usize, is_header: bool, apply_to: u8, hf_para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.split_paragraph_in_header_footer_native(section_idx, is_header, apply_to, hf_para_idx, char_offset)
    }
    #[inline]
    pub fn split_paragraph(&mut self, section_idx: usize, para_idx: usize, char_offset: usize) -> Result<String, HwpError> {
        self.split_paragraph_native(section_idx, para_idx, char_offset)
    }
    #[inline]
    pub fn split_table_cell_into(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, row: u16, col: u16, n_rows: u16, m_cols: u16, equal_row_height: bool, merge_first: bool) -> Result<String, HwpError> {
        self.split_table_cell_into_native(section_idx, parent_para_idx, control_idx, row, col, n_rows, m_cols, equal_row_height, merge_first)
    }
    #[inline]
    pub fn split_table_cell(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, row: u16, col: u16) -> Result<String, HwpError> {
        self.split_table_cell_native(section_idx, parent_para_idx, control_idx, row, col)
    }
    #[inline]
    pub fn split_table_cells_in_range(&mut self, section_idx: usize, parent_para_idx: usize, control_idx: usize, start_row: u16, start_col: u16, end_row: u16, end_col: u16, n_rows: u16, m_cols: u16, equal_row_height: bool) -> Result<String, HwpError> {
        self.split_table_cells_in_range_native(section_idx, parent_para_idx, control_idx, start_row, start_col, end_row, end_col, n_rows, m_cols, equal_row_height)
    }
    #[inline]
    pub fn toggle_hide_header_footer(&mut self, page_num: u32, is_header: bool) -> Result<String, HwpError> {
        self.toggle_hide_header_footer_native(page_num, is_header)
    }
    #[inline]
    pub fn ungroup_shape(&mut self, section_idx: usize, para_idx: usize, control_idx: usize) -> Result<String, HwpError> {
        self.ungroup_shape_native(section_idx, para_idx, control_idx)
    }
}
