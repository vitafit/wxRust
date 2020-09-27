/* added by build.rs */
use libc::*;

pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_longlong;
pub type __uint64_t = ::libc::c_ulonglong;
pub type __darwin_intptr_t = ::libc::c_long;
pub type __darwin_natural_t = ::libc::c_uint;
pub type __darwin_ct_rune_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 16usize],
}
impl Union_Unnamed1 {
    pub unsafe fn __mbstate8(&mut self) -> *mut [::libc::c_char; 128usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn _mbstateL(&mut self) -> *mut ::libc::c_longlong {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::clone::Clone for Union_Unnamed1 {
    fn clone(&self) -> Union_Unnamed1 { *self }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Union_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type __mbstate_t = Union_Unnamed1;
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::libc::c_long;
pub type __darwin_size_t = ::libc::c_ulong;
pub type __darwin_wchar_t = ::libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::libc::c_int;
pub type __darwin_clock_t = ::libc::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::libc::c_long;
pub type __darwin_time_t = ::libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::libc::c_uint;
pub type __darwin_fsfilcnt_t = ::libc::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::libc::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::libc::c_char; 37usize];
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut ::libc::c_void)
                                             -> ()>,
    pub __arg: *mut ::libc::c_void,
    pub __next: *mut Struct___darwin_pthread_handler_rec,
}
impl ::std::clone::Clone for Struct___darwin_pthread_handler_rec {
    fn clone(&self) -> Struct___darwin_pthread_handler_rec { *self }
}
impl ::std::default::Default for Struct___darwin_pthread_handler_rec {
    fn default() -> Struct___darwin_pthread_handler_rec {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_attr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 56usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_attr_t {
    fn clone(&self) -> Struct__opaque_pthread_attr_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_attr_t {
    fn default() -> Struct__opaque_pthread_attr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_cond_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 40usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_cond_t {
    fn clone(&self) -> Struct__opaque_pthread_cond_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_cond_t {
    fn default() -> Struct__opaque_pthread_cond_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_condattr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 8usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_condattr_t {
    fn clone(&self) -> Struct__opaque_pthread_condattr_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_condattr_t {
    fn default() -> Struct__opaque_pthread_condattr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_mutex_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 56usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_mutex_t {
    fn clone(&self) -> Struct__opaque_pthread_mutex_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_mutex_t {
    fn default() -> Struct__opaque_pthread_mutex_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_mutexattr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 8usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_mutexattr_t {
    fn clone(&self) -> Struct__opaque_pthread_mutexattr_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_mutexattr_t {
    fn default() -> Struct__opaque_pthread_mutexattr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_once_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 8usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_once_t {
    fn clone(&self) -> Struct__opaque_pthread_once_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_once_t {
    fn default() -> Struct__opaque_pthread_once_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_rwlock_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 192usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_rwlock_t {
    fn clone(&self) -> Struct__opaque_pthread_rwlock_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_rwlock_t {
    fn default() -> Struct__opaque_pthread_rwlock_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_rwlockattr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 16usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_rwlockattr_t {
    fn clone(&self) -> Struct__opaque_pthread_rwlockattr_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_rwlockattr_t {
    fn default() -> Struct__opaque_pthread_rwlockattr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_t {
    pub __sig: ::libc::c_long,
    pub __cleanup_stack: *mut Struct___darwin_pthread_handler_rec,
    pub __opaque: [::libc::c_char; 8176usize],
}
impl ::std::clone::Clone for Struct__opaque_pthread_t {
    fn clone(&self) -> Struct__opaque_pthread_t { *self }
}
impl ::std::default::Default for Struct__opaque_pthread_t {
    fn default() -> Struct__opaque_pthread_t {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __darwin_pthread_attr_t = Struct__opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = Struct__opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = Struct__opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::libc::c_ulong;
pub type __darwin_pthread_mutex_t = Struct__opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = Struct__opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = Struct__opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = Struct__opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = Struct__opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut Struct__opaque_pthread_t;
pub type __darwin_nl_item = ::libc::c_int;
pub type __darwin_wctrans_t = ::libc::c_int;
pub type __darwin_wctype_t = __uint32_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: ::libc::c_long,
}
impl ::std::clone::Clone for Struct_timespec {
    fn clone(&self) -> Struct_timespec { *self }
}
impl ::std::default::Default for Struct_timespec {
    fn default() -> Struct_timespec { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tm {
    pub tm_sec: ::libc::c_int,
    pub tm_min: ::libc::c_int,
    pub tm_hour: ::libc::c_int,
    pub tm_mday: ::libc::c_int,
    pub tm_mon: ::libc::c_int,
    pub tm_year: ::libc::c_int,
    pub tm_wday: ::libc::c_int,
    pub tm_yday: ::libc::c_int,
    pub tm_isdst: ::libc::c_int,
    pub tm_gmtoff: ::libc::c_long,
    pub tm_zone: *mut ::libc::c_char,
}
impl ::std::clone::Clone for Struct_tm {
    fn clone(&self) -> Struct_tm { *self }
}
impl ::std::default::Default for Struct_tm {
    fn default() -> Struct_tm { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut tzname: *mut *mut ::libc::c_char;
    pub static mut getdate_err: ::libc::c_int;
    pub static mut timezone: ::libc::c_long;
    pub static mut daylight: ::libc::c_int;
}
extern "C" {
    pub fn Null_AcceleratorTable() -> *mut ::libc::c_void;
    pub fn Null_Bitmap() -> *mut ::libc::c_void;
    pub fn Null_Brush() -> *mut ::libc::c_void;
    pub fn Null_Colour() -> *mut ::libc::c_void;
    pub fn Null_Cursor() -> *mut ::libc::c_void;
    pub fn Null_Font() -> *mut ::libc::c_void;
    pub fn Null_Icon() -> *mut ::libc::c_void;
    pub fn Null_Palette() -> *mut ::libc::c_void;
    pub fn Null_Pen() -> *mut ::libc::c_void;
    pub fn expEVT_COMMAND_AUITOOLBAR_TOOL_DROPDOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUITOOLBAR_OVERFLOW_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUITOOLBAR_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUITOOLBAR_MIDDLE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUITOOLBAR_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CLOSE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CLOSED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_BUTTON() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_END_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_DRAG_MOTION() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_ALLOW_DND() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_MIDDLE_DOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_MIDDLE_UP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_RIGHT_DOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_RIGHT_UP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_DRAG_DONE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_AUINOTEBOOK_BG_DCLICK() -> ::libc::c_int;
    pub fn expEVT_AUI_PANE_BUTTON() -> ::libc::c_int;
    pub fn expEVT_AUI_PANE_CLOSE() -> ::libc::c_int;
    pub fn expEVT_AUI_PANE_MAXIMIZE() -> ::libc::c_int;
    pub fn expEVT_AUI_PANE_RESTORE() -> ::libc::c_int;
    pub fn expEVT_AUI_RENDER() -> ::libc::c_int;
    pub fn expEVT_AUI_FIND_MANAGER() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_SEL_CHANGED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_DOUBLECLICKED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_WEEKDAY_CLICKED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_WEEK_CLICKED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_DAY_CHANGED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_MONTH_CHANGED() -> ::libc::c_int;
    pub fn expEVT_CALENDAR_YEAR_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_CHOICEBOOK_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_CHOICEBOOK_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_CLIPBOARD_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_COLOURPICKER_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_COLLPANE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_SELECTION_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_ACTIVATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_COLLAPSED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EXPANDED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_COLLAPSING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EXPANDING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_START_EDITING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EDITING_STARTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EDITING_DONE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_VALUE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_CONTEXT_MENU() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_HEADER_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK()
     -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_SORTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_REORDERED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_CACHE_HINT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_DROP_POSSIBLE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_DROP() -> ::libc::c_int;
    pub fn expEVT_DATE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_WINDOW_MODAL_DIALOG_CLOSED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_BUTTON_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_CHECKBOX_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_CHOICE_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LISTBOX_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LISTBOX_DOUBLECLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_CHECKLISTBOX_TOGGLED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_MENU_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SLIDER_UPDATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RADIOBOX_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RADIOBUTTON_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_VLBOX_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_COMBOBOX_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOOL_RCLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOOL_DROPDOWN_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOOL_ENTER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_COMBOBOX_DROPDOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_COMBOBOX_CLOSEUP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_THREAD() -> ::libc::c_int;
    pub fn expEVT_LEFT_DOWN() -> ::libc::c_int;
    pub fn expEVT_LEFT_UP() -> ::libc::c_int;
    pub fn expEVT_MIDDLE_DOWN() -> ::libc::c_int;
    pub fn expEVT_MIDDLE_UP() -> ::libc::c_int;
    pub fn expEVT_RIGHT_DOWN() -> ::libc::c_int;
    pub fn expEVT_RIGHT_UP() -> ::libc::c_int;
    pub fn expEVT_MOTION() -> ::libc::c_int;
    pub fn expEVT_ENTER_WINDOW() -> ::libc::c_int;
    pub fn expEVT_LEAVE_WINDOW() -> ::libc::c_int;
    pub fn expEVT_LEFT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_MIDDLE_DCLICK() -> ::libc::c_int;
    pub fn expEVT_RIGHT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_SET_FOCUS() -> ::libc::c_int;
    pub fn expEVT_KILL_FOCUS() -> ::libc::c_int;
    pub fn expEVT_CHILD_FOCUS() -> ::libc::c_int;
    pub fn expEVT_MOUSEWHEEL() -> ::libc::c_int;
    pub fn expEVT_AUX1_DOWN() -> ::libc::c_int;
    pub fn expEVT_AUX1_UP() -> ::libc::c_int;
    pub fn expEVT_AUX1_DCLICK() -> ::libc::c_int;
    pub fn expEVT_AUX2_DOWN() -> ::libc::c_int;
    pub fn expEVT_AUX2_UP() -> ::libc::c_int;
    pub fn expEVT_AUX2_DCLICK() -> ::libc::c_int;
    pub fn expEVT_CHAR() -> ::libc::c_int;
    pub fn expEVT_CHAR_HOOK() -> ::libc::c_int;
    pub fn expEVT_NAVIGATION_KEY() -> ::libc::c_int;
    pub fn expEVT_KEY_DOWN() -> ::libc::c_int;
    pub fn expEVT_KEY_UP() -> ::libc::c_int;
    pub fn expEVT_HOTKEY() -> ::libc::c_int;
    pub fn expEVT_SET_CURSOR() -> ::libc::c_int;
    pub fn expEVT_SCROLL_TOP() -> ::libc::c_int;
    pub fn expEVT_SCROLL_BOTTOM() -> ::libc::c_int;
    pub fn expEVT_SCROLL_LINEUP() -> ::libc::c_int;
    pub fn expEVT_SCROLL_LINEDOWN() -> ::libc::c_int;
    pub fn expEVT_SCROLL_PAGEUP() -> ::libc::c_int;
    pub fn expEVT_SCROLL_PAGEDOWN() -> ::libc::c_int;
    pub fn expEVT_SCROLL_THUMBTRACK() -> ::libc::c_int;
    pub fn expEVT_SCROLL_THUMBRELEASE() -> ::libc::c_int;
    pub fn expEVT_SCROLL_CHANGED() -> ::libc::c_int;
    pub fn expEVT_SPIN_UP() -> ::libc::c_int;
    pub fn expEVT_SPIN_DOWN() -> ::libc::c_int;
    pub fn expEVT_SPIN() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_TOP() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_BOTTOM() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_LINEUP() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_LINEDOWN() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_PAGEUP() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_PAGEDOWN() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_THUMBTRACK() -> ::libc::c_int;
    pub fn expEVT_SCROLLWIN_THUMBRELEASE() -> ::libc::c_int;
    pub fn expEVT_SIZE() -> ::libc::c_int;
    pub fn expEVT_MOVE() -> ::libc::c_int;
    pub fn expEVT_CLOSE_WINDOW() -> ::libc::c_int;
    pub fn expEVT_END_SESSION() -> ::libc::c_int;
    pub fn expEVT_QUERY_END_SESSION() -> ::libc::c_int;
    pub fn expEVT_ACTIVATE_APP() -> ::libc::c_int;
    pub fn expEVT_ACTIVATE() -> ::libc::c_int;
    pub fn expEVT_CREATE() -> ::libc::c_int;
    pub fn expEVT_DESTROY() -> ::libc::c_int;
    pub fn expEVT_SHOW() -> ::libc::c_int;
    pub fn expEVT_ICONIZE() -> ::libc::c_int;
    pub fn expEVT_MAXIMIZE() -> ::libc::c_int;
    pub fn expEVT_MOUSE_CAPTURE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_MOUSE_CAPTURE_LOST() -> ::libc::c_int;
    pub fn expEVT_PAINT() -> ::libc::c_int;
    pub fn expEVT_ERASE_BACKGROUND() -> ::libc::c_int;
    pub fn expEVT_NC_PAINT() -> ::libc::c_int;
    pub fn expEVT_MENU_OPEN() -> ::libc::c_int;
    pub fn expEVT_MENU_CLOSE() -> ::libc::c_int;
    pub fn expEVT_MENU_HIGHLIGHT() -> ::libc::c_int;
    pub fn expEVT_CONTEXT_MENU() -> ::libc::c_int;
    pub fn expEVT_SYS_COLOUR_CHANGED() -> ::libc::c_int;
    pub fn expEVT_DISPLAY_CHANGED() -> ::libc::c_int;
    pub fn expEVT_QUERY_NEW_PALETTE() -> ::libc::c_int;
    pub fn expEVT_PALETTE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_JOY_BUTTON_DOWN() -> ::libc::c_int;
    pub fn expEVT_JOY_BUTTON_UP() -> ::libc::c_int;
    pub fn expEVT_JOY_MOVE() -> ::libc::c_int;
    pub fn expEVT_JOY_ZMOVE() -> ::libc::c_int;
    pub fn expEVT_DROP_FILES() -> ::libc::c_int;
    pub fn expEVT_INIT_DIALOG() -> ::libc::c_int;
    pub fn expEVT_IDLE() -> ::libc::c_int;
    pub fn expEVT_UPDATE_UI() -> ::libc::c_int;
    pub fn expEVT_SIZING() -> ::libc::c_int;
    pub fn expEVT_MOVING() -> ::libc::c_int;
    pub fn expEVT_MOVE_START() -> ::libc::c_int;
    pub fn expEVT_MOVE_END() -> ::libc::c_int;
    pub fn expEVT_HIBERNATE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_COPY() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_CUT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_PASTE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LEFT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LEFT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIGHT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SET_FOCUS() -> ::libc::c_int;
    pub fn expEVT_COMMAND_KILL_FOCUS() -> ::libc::c_int;
    pub fn expEVT_COMMAND_ENTER() -> ::libc::c_int;
    pub fn expEVT_HELP() -> ::libc::c_int;
    pub fn expEVT_DETAILED_HELP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOOL_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FIND() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FIND_NEXT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FIND_REPLACE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FIND_REPLACE_ALL() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FIND_CLOSE() -> ::libc::c_int;
    pub fn expEVT_FILECTRL_SELECTIONCHANGED() -> ::libc::c_int;
    pub fn expEVT_FILECTRL_FILEACTIVATED() -> ::libc::c_int;
    pub fn expEVT_FILECTRL_FOLDERCHANGED() -> ::libc::c_int;
    pub fn expEVT_FILECTRL_FILTERCHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FILEPICKER_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_DIRPICKER_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_FONTPICKER_CHANGED() -> ::libc::c_int;
    pub fn expEVT_FSWATCHER() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_LEFT_CLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_LEFT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_RIGHT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_LABEL_LEFT_CLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_LABEL_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_LABEL_LEFT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_LABEL_RIGHT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_GRID_ROW_SIZE() -> ::libc::c_int;
    pub fn expEVT_GRID_COL_SIZE() -> ::libc::c_int;
    pub fn expEVT_GRID_RANGE_SELECT() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_CHANGING() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_CHANGED() -> ::libc::c_int;
    pub fn expEVT_GRID_SELECT_CELL() -> ::libc::c_int;
    pub fn expEVT_GRID_EDITOR_SHOWN() -> ::libc::c_int;
    pub fn expEVT_GRID_EDITOR_HIDDEN() -> ::libc::c_int;
    pub fn expEVT_GRID_EDITOR_CREATED() -> ::libc::c_int;
    pub fn expEVT_GRID_CELL_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_GRID_COL_MOVE() -> ::libc::c_int;
    pub fn expEVT_GRID_COL_SORT() -> ::libc::c_int;
    pub fn expEVT_QUERY_LAYOUT_INFO() -> ::libc::c_int;
    pub fn expEVT_CALCULATE_LAYOUT() -> ::libc::c_int;
    pub fn expEVT_SASH_DRAGGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_MIDDLE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_RIGHT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_MIDDLE_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_SEPARATOR_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_BEGIN_RESIZE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_RESIZING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_END_RESIZE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_BEGIN_REORDER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_END_REORDER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HEADER_DRAGGING_CANCELLED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HTML_CELL_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HTML_CELL_HOVER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HTML_LINK_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_HYPERLINK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_BEGIN_RDRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_BEGIN_LABEL_EDIT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_END_LABEL_EDIT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_DELETE_ITEM() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_DELETE_ALL_ITEMS() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_ITEM_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_ITEM_DESELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_KEY_DOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_INSERT_ITEM() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_COL_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_ITEM_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_ITEM_MIDDLE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_ITEM_ACTIVATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_CACHE_HINT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_COL_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_COL_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_COL_DRAGGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_COL_END_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LIST_ITEM_FOCUSED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LISTBOOK_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_LISTBOOK_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_NOTEBOOK_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_NOTEBOOK_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_POWER_SUSPENDING() -> ::libc::c_int;
    pub fn expEVT_POWER_SUSPENDED() -> ::libc::c_int;
    pub fn expEVT_POWER_SUSPEND_CANCEL() -> ::libc::c_int;
    pub fn expEVT_POWER_RESUME() -> ::libc::c_int;
    pub fn expEVT_END_PROCESS() -> ::libc::c_int;
    pub fn expEVT_PG_SELECTED() -> ::libc::c_int;
    pub fn expEVT_PG_CHANGING() -> ::libc::c_int;
    pub fn expEVT_PG_CHANGED() -> ::libc::c_int;
    pub fn expEVT_PG_HIGHLIGHTED() -> ::libc::c_int;
    pub fn expEVT_PG_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_PG_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_PG_ITEM_COLLAPSED() -> ::libc::c_int;
    pub fn expEVT_PG_ITEM_EXPANDED() -> ::libc::c_int;
    pub fn expEVT_PG_DOUBLE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBAR_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBAR_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_MIDDLE_DOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_MIDDLE_UP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_RIGHT_DOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_RIGHT_UP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBUTTON_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONBUTTON_DROPDOWN_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONGALLERY_HOVER_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONGALLERY_SELECTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONTOOL_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RIBBONTOOL_DROPDOWN_CLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_LEFT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_MIDDLE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_LEFT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_RETURN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_CHARACTER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_DELETE() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_REPLACING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_REPLACED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_CONTENT_INSERTED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_CONTENT_DELETED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_STYLE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_SELECTION_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_RICHTEXT_BUFFER_RESET() -> ::libc::c_int;
    pub fn expEVT_SOCKET() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SPINCTRL_UPDATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SPINCTRLDOUBLE_UPDATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SPLITTER_SASH_POS_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SPLITTER_SASH_POS_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SPLITTER_DOUBLECLICKED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SPLITTER_UNSPLIT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SEARCHCTRL_CANCEL_BTN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_SEARCHCTRL_SEARCH_BTN() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_MOVE() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_LEFT_DOWN() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_LEFT_UP() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_RIGHT_DOWN() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_RIGHT_UP() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_LEFT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_RIGHT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_BALLOON_TIMEOUT() -> ::libc::c_int;
    pub fn expEVT_TASKBAR_BALLOON_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_UPDATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_ENTER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_URL() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TEXT_MAXLEN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOGGLEBUTTON_CLICKED() -> ::libc::c_int;
    pub fn expEVT_TIMER() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOOLBOOK_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TOOLBOOK_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_BEGIN_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_BEGIN_RDRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_BEGIN_LABEL_EDIT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_END_LABEL_EDIT() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_DELETE_ITEM() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_GET_INFO() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_SET_INFO() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_EXPANDED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_EXPANDING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_COLLAPSED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_COLLAPSING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_SEL_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_SEL_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_KEY_DOWN() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_ACTIVATED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_RIGHT_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_MIDDLE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_END_DRAG() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_STATE_IMAGE_CLICK() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_GETTOOLTIP() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREE_ITEM_MENU() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREEBOOK_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREEBOOK_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREEBOOK_NODE_COLLAPSED() -> ::libc::c_int;
    pub fn expEVT_COMMAND_TREEBOOK_NODE_EXPANDED() -> ::libc::c_int;
    pub fn expEVT_WIZARD_PAGE_CHANGED() -> ::libc::c_int;
    pub fn expEVT_WIZARD_PAGE_CHANGING() -> ::libc::c_int;
    pub fn expEVT_WIZARD_CANCEL() -> ::libc::c_int;
    pub fn expEVT_WIZARD_HELP() -> ::libc::c_int;
    pub fn expEVT_WIZARD_FINISHED() -> ::libc::c_int;
    pub fn expEVT_WIZARD_PAGE_SHOWN() -> ::libc::c_int;
    pub fn expEVT_DELETE() -> ::libc::c_int;
    pub fn expEVT_HTML_CELL_CLICKED() -> ::libc::c_int;
    pub fn expEVT_HTML_CELL_MOUSE_HOVER() -> ::libc::c_int;
    pub fn expEVT_HTML_LINK_CLICKED() -> ::libc::c_int;
    pub fn expEVT_HTML_SET_TITLE() -> ::libc::c_int;
    pub fn expEVT_INPUT_SINK() -> ::libc::c_int;
    pub fn expEVT_SORT() -> ::libc::c_int;
    pub fn expK_BACK() -> ::libc::c_int;
    pub fn expK_TAB() -> ::libc::c_int;
    pub fn expK_RETURN() -> ::libc::c_int;
    pub fn expK_ESCAPE() -> ::libc::c_int;
    pub fn expK_SPACE() -> ::libc::c_int;
    pub fn expK_DELETE() -> ::libc::c_int;
    pub fn expK_START() -> ::libc::c_int;
    pub fn expK_LBUTTON() -> ::libc::c_int;
    pub fn expK_RBUTTON() -> ::libc::c_int;
    pub fn expK_CANCEL() -> ::libc::c_int;
    pub fn expK_MBUTTON() -> ::libc::c_int;
    pub fn expK_CLEAR() -> ::libc::c_int;
    pub fn expK_SHIFT() -> ::libc::c_int;
    pub fn expK_ALT() -> ::libc::c_int;
    pub fn expK_CONTROL() -> ::libc::c_int;
    pub fn expK_MENU() -> ::libc::c_int;
    pub fn expK_PAUSE() -> ::libc::c_int;
    pub fn expK_CAPITAL() -> ::libc::c_int;
    pub fn expK_END() -> ::libc::c_int;
    pub fn expK_HOME() -> ::libc::c_int;
    pub fn expK_LEFT() -> ::libc::c_int;
    pub fn expK_UP() -> ::libc::c_int;
    pub fn expK_RIGHT() -> ::libc::c_int;
    pub fn expK_DOWN() -> ::libc::c_int;
    pub fn expK_SELECT() -> ::libc::c_int;
    pub fn expK_PRINT() -> ::libc::c_int;
    pub fn expK_EXECUTE() -> ::libc::c_int;
    pub fn expK_SNAPSHOT() -> ::libc::c_int;
    pub fn expK_INSERT() -> ::libc::c_int;
    pub fn expK_HELP() -> ::libc::c_int;
    pub fn expK_NUMPAD0() -> ::libc::c_int;
    pub fn expK_NUMPAD1() -> ::libc::c_int;
    pub fn expK_NUMPAD2() -> ::libc::c_int;
    pub fn expK_NUMPAD3() -> ::libc::c_int;
    pub fn expK_NUMPAD4() -> ::libc::c_int;
    pub fn expK_NUMPAD5() -> ::libc::c_int;
    pub fn expK_NUMPAD6() -> ::libc::c_int;
    pub fn expK_NUMPAD7() -> ::libc::c_int;
    pub fn expK_NUMPAD8() -> ::libc::c_int;
    pub fn expK_NUMPAD9() -> ::libc::c_int;
    pub fn expK_MULTIPLY() -> ::libc::c_int;
    pub fn expK_ADD() -> ::libc::c_int;
    pub fn expK_SEPARATOR() -> ::libc::c_int;
    pub fn expK_SUBTRACT() -> ::libc::c_int;
    pub fn expK_DECIMAL() -> ::libc::c_int;
    pub fn expK_DIVIDE() -> ::libc::c_int;
    pub fn expK_F1() -> ::libc::c_int;
    pub fn expK_F2() -> ::libc::c_int;
    pub fn expK_F3() -> ::libc::c_int;
    pub fn expK_F4() -> ::libc::c_int;
    pub fn expK_F5() -> ::libc::c_int;
    pub fn expK_F6() -> ::libc::c_int;
    pub fn expK_F7() -> ::libc::c_int;
    pub fn expK_F8() -> ::libc::c_int;
    pub fn expK_F9() -> ::libc::c_int;
    pub fn expK_F10() -> ::libc::c_int;
    pub fn expK_F11() -> ::libc::c_int;
    pub fn expK_F12() -> ::libc::c_int;
    pub fn expK_F13() -> ::libc::c_int;
    pub fn expK_F14() -> ::libc::c_int;
    pub fn expK_F15() -> ::libc::c_int;
    pub fn expK_F16() -> ::libc::c_int;
    pub fn expK_F17() -> ::libc::c_int;
    pub fn expK_F18() -> ::libc::c_int;
    pub fn expK_F19() -> ::libc::c_int;
    pub fn expK_F20() -> ::libc::c_int;
    pub fn expK_F21() -> ::libc::c_int;
    pub fn expK_F22() -> ::libc::c_int;
    pub fn expK_F23() -> ::libc::c_int;
    pub fn expK_F24() -> ::libc::c_int;
    pub fn expK_NUMLOCK() -> ::libc::c_int;
    pub fn expK_SCROLL() -> ::libc::c_int;
    pub fn expK_PAGEUP() -> ::libc::c_int;
    pub fn expK_PAGEDOWN() -> ::libc::c_int;
    pub fn expK_NUMPAD_SPACE() -> ::libc::c_int;
    pub fn expK_NUMPAD_TAB() -> ::libc::c_int;
    pub fn expK_NUMPAD_ENTER() -> ::libc::c_int;
    pub fn expK_NUMPAD_F1() -> ::libc::c_int;
    pub fn expK_NUMPAD_F2() -> ::libc::c_int;
    pub fn expK_NUMPAD_F3() -> ::libc::c_int;
    pub fn expK_NUMPAD_F4() -> ::libc::c_int;
    pub fn expK_NUMPAD_HOME() -> ::libc::c_int;
    pub fn expK_NUMPAD_LEFT() -> ::libc::c_int;
    pub fn expK_NUMPAD_UP() -> ::libc::c_int;
    pub fn expK_NUMPAD_RIGHT() -> ::libc::c_int;
    pub fn expK_NUMPAD_DOWN() -> ::libc::c_int;
    pub fn expK_NUMPAD_PAGEUP() -> ::libc::c_int;
    pub fn expK_NUMPAD_PAGEDOWN() -> ::libc::c_int;
    pub fn expK_NUMPAD_END() -> ::libc::c_int;
    pub fn expK_NUMPAD_BEGIN() -> ::libc::c_int;
    pub fn expK_NUMPAD_INSERT() -> ::libc::c_int;
    pub fn expK_NUMPAD_DELETE() -> ::libc::c_int;
    pub fn expK_NUMPAD_EQUAL() -> ::libc::c_int;
    pub fn expK_NUMPAD_MULTIPLY() -> ::libc::c_int;
    pub fn expK_NUMPAD_ADD() -> ::libc::c_int;
    pub fn expK_NUMPAD_SEPARATOR() -> ::libc::c_int;
    pub fn expK_NUMPAD_SUBTRACT() -> ::libc::c_int;
    pub fn expK_NUMPAD_DECIMAL() -> ::libc::c_int;
    pub fn expK_NUMPAD_DIVIDE() -> ::libc::c_int;
    pub fn ELJSysErrorCode() -> ::libc::c_int;
    pub fn ELJSysErrorMsg(nErrCode: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn LogErrorMsg(_msg: *mut ::libc::c_void) -> ();
    pub fn LogFatalErrorMsg(_msg: *mut ::libc::c_void) -> ();
    pub fn LogMessageMsg(_msg: *mut ::libc::c_void) -> ();
    pub fn LogWarningMsg(_msg: *mut ::libc::c_void) -> ();
    pub fn Quantize(src: *mut ::libc::c_void, dest: *mut ::libc::c_void,
                    desiredNoColours: ::libc::c_int,
                    eightBitData: *mut ::libc::c_void, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn QuantizePalette(src: *mut ::libc::c_void,
                           dest: *mut ::libc::c_void,
                           pPalette: *mut ::libc::c_void,
                           desiredNoColours: ::libc::c_int,
                           eightBitData: *mut ::libc::c_void,
                           flags: ::libc::c_int) -> ::libc::c_int;
    pub fn wxCFree(_ptr: *mut ::libc::c_void) -> ();
    pub fn wxGetELJLocale() -> *mut ::libc::c_void;
    pub fn wxGetELJTranslation(sz: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMutexGui_Enter() -> ();
    pub fn wxMutexGui_Leave() -> ();
    pub fn ELJApp_Bell() -> ();
    pub fn ELJApp_CreateLogTarget() -> *mut ::libc::c_void;
    pub fn ELJApp_Dispatch() -> ();
    pub fn ELJApp_DisplaySize() -> *mut ::libc::c_void;
    pub fn ELJApp_EnableTooltips(_enable: ::libc::c_int) -> ();
    pub fn ELJApp_EnableTopLevelWindows(_enb: ::libc::c_int) -> ();
    pub fn ELJApp_ExecuteProcess(_cmd: *mut ::libc::c_void,
                                 _snc: ::libc::c_int,
                                 _prc: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJApp_Exit() -> ();
    pub fn ELJApp_ExitMainLoop() -> ();
    pub fn ELJApp_FindWindowById(_id: ::libc::c_int,
                                 _prt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJApp_FindWindowByLabel(_lbl: *mut ::libc::c_void,
                                    _prt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJApp_FindWindowByName(_lbl: *mut ::libc::c_void,
                                   _prt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJApp_GetApp() -> *mut ::libc::c_void;
    pub fn ELJApp_GetAppName() -> *mut ::libc::c_void;
    pub fn ELJApp_GetClassName() -> *mut ::libc::c_void;
    pub fn ELJApp_GetExitOnFrameDelete() -> ::libc::c_int;
    pub fn ELJApp_GetOsDescription() -> *mut ::libc::c_void;
    pub fn ELJApp_GetOsVersion(_maj: *mut ::libc::c_void,
                               _min: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJApp_GetTopWindow() -> *mut ::libc::c_void;
    pub fn ELJApp_GetUseBestVisual() -> ::libc::c_int;
    pub fn ELJApp_GetUserHome(_usr: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJApp_GetUserId() -> *mut ::libc::c_void;
    pub fn ELJApp_GetUserName() -> *mut ::libc::c_void;
    pub fn ELJApp_GetVendorName() -> *mut ::libc::c_void;
    pub fn ELJApp_InitAllImageHandlers() -> ();
    pub fn ELJApp_Initialized() -> ::libc::c_int;
    pub fn ELJApp_MainLoop() -> ::libc::c_int;
    pub fn ELJApp_MousePosition() -> *mut ::libc::c_void;
    pub fn ELJApp_Pending() -> ::libc::c_int;
    pub fn ELJApp_SafeYield(_win: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJApp_SetAppName(name: *mut ::libc::c_void) -> ();
    pub fn ELJApp_SetClassName(name: *mut ::libc::c_void) -> ();
    pub fn ELJApp_SetExitOnFrameDelete(flag: ::libc::c_int) -> ();
    pub fn ELJApp_SetPrintMode(mode: ::libc::c_int) -> ();
    pub fn ELJApp_SetTooltipDelay(_ms: ::libc::c_int) -> ();
    pub fn ELJApp_SetTopWindow(_wnd: *mut ::libc::c_void) -> ();
    pub fn ELJApp_SetUseBestVisual(flag: ::libc::c_int) -> ();
    pub fn ELJApp_SetVendorName(name: *mut ::libc::c_void) -> ();
    pub fn ELJApp_Sleep(_scs: ::libc::c_int) -> ();
    pub fn ELJApp_MilliSleep(_mscs: ::libc::c_int) -> ();
    pub fn ELJApp_Yield() -> ::libc::c_int;
    pub fn ELJApp_IsTerminating() -> ::libc::c_int;
    pub fn ELJArtProv_Create(_obj: *mut ::libc::c_void,
                             _clb: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJArtProv_Release(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJClient_Create(_eobj: *mut ::libc::c_void,
                            _cnct: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJClient_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJClient_MakeConnection(_obj: *mut ::libc::c_void,
                                    host: *mut ::libc::c_void,
                                    server: *mut ::libc::c_void,
                                    topic: *mut ::libc::c_void) -> ();
    pub fn ELJCommand_CanUndo(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJCommand_Create(_und: ::libc::c_int, _nme: *mut ::libc::c_void,
                             _obj: *mut ::libc::c_void,
                             _clb: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJCommand_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJCommand_GetName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJConnection_Advise(_obj: *mut ::libc::c_void,
                                item: *mut ::libc::c_void,
                                data: *mut ::libc::c_void,
                                size: ::libc::c_int, format: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ELJConnection_Compress(_obj: *mut ::libc::c_void,
                                  on: ::libc::c_int) -> ();
    pub fn ELJConnection_Create(_obj: *mut ::libc::c_void,
                                buffer: *mut ::libc::c_void,
                                size: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn ELJConnection_CreateDefault(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJConnection_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_Disconnect(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ELJConnection_Execute(_obj: *mut ::libc::c_void,
                                 data: *mut ::libc::c_void,
                                 size: ::libc::c_int, format: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ELJConnection_Poke(_obj: *mut ::libc::c_void,
                              item: *mut ::libc::c_void,
                              data: *mut ::libc::c_void, size: ::libc::c_int,
                              format: ::libc::c_int) -> ::libc::c_int;
    pub fn ELJConnection_Request(_obj: *mut ::libc::c_void,
                                 item: *mut ::libc::c_void,
                                 size: *mut ::libc::c_void,
                                 format: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn ELJConnection_SetOnAdvise(_obj: *mut ::libc::c_void,
                                     _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_SetOnDisconnect(_obj: *mut ::libc::c_void,
                                         _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_SetOnExecute(_obj: *mut ::libc::c_void,
                                      _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_SetOnPoke(_obj: *mut ::libc::c_void,
                                   _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_SetOnRequest(_obj: *mut ::libc::c_void,
                                      _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_SetOnStartAdvise(_obj: *mut ::libc::c_void,
                                          _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_SetOnStopAdvise(_obj: *mut ::libc::c_void,
                                         _fnc: *mut ::libc::c_void) -> ();
    pub fn ELJConnection_StartAdvise(_obj: *mut ::libc::c_void,
                                     item: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ELJConnection_StopAdvise(_obj: *mut ::libc::c_void,
                                    item: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ELJDragDataObject_Create(_obj: *mut ::libc::c_void,
                                    _fmt: *mut ::libc::c_void,
                                    _func1: *mut ::libc::c_void,
                                    _func2: *mut ::libc::c_void,
                                    _func3: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJDragDataObject_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJDropTarget_Create(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJDropTarget_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJDropTarget_SetOnData(_obj: *mut ::libc::c_void,
                                   _func: *mut ::libc::c_void) -> ();
    pub fn ELJDropTarget_SetOnDragOver(_obj: *mut ::libc::c_void,
                                       _func: *mut ::libc::c_void) -> ();
    pub fn ELJDropTarget_SetOnDrop(_obj: *mut ::libc::c_void,
                                   _func: *mut ::libc::c_void) -> ();
    pub fn ELJDropTarget_SetOnEnter(_obj: *mut ::libc::c_void,
                                    _func: *mut ::libc::c_void) -> ();
    pub fn ELJDropTarget_SetOnLeave(_obj: *mut ::libc::c_void,
                                    _func: *mut ::libc::c_void) -> ();
    pub fn ELJFileDropTarget_Create(_obj: *mut ::libc::c_void,
                                    _func: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJFileDropTarget_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJFileDropTarget_SetOnData(_obj: *mut ::libc::c_void,
                                       _func: *mut ::libc::c_void) -> ();
    pub fn ELJFileDropTarget_SetOnDragOver(_obj: *mut ::libc::c_void,
                                           _func: *mut ::libc::c_void) -> ();
    pub fn ELJFileDropTarget_SetOnDrop(_obj: *mut ::libc::c_void,
                                       _func: *mut ::libc::c_void) -> ();
    pub fn ELJFileDropTarget_SetOnEnter(_obj: *mut ::libc::c_void,
                                        _func: *mut ::libc::c_void) -> ();
    pub fn ELJFileDropTarget_SetOnLeave(_obj: *mut ::libc::c_void,
                                        _func: *mut ::libc::c_void) -> ();
    pub fn ELJGridTable_Create(_obj: *mut ::libc::c_void,
                               _EifGetNumberRows: *mut ::libc::c_void,
                               _EifGetNumberCols: *mut ::libc::c_void,
                               _EifGetValue: *mut ::libc::c_void,
                               _EifSetValue: *mut ::libc::c_void,
                               _EifIsEmptyCell: *mut ::libc::c_void,
                               _EifClear: *mut ::libc::c_void,
                               _EifInsertRows: *mut ::libc::c_void,
                               _EifAppendRows: *mut ::libc::c_void,
                               _EifDeleteRows: *mut ::libc::c_void,
                               _EifInsertCols: *mut ::libc::c_void,
                               _EifAppendCols: *mut ::libc::c_void,
                               _EifDeleteCols: *mut ::libc::c_void,
                               _EifSetRowLabelValue: *mut ::libc::c_void,
                               _EifSetColLabelValue: *mut ::libc::c_void,
                               _EifGetRowLabelValue: *mut ::libc::c_void,
                               _EifGetColLabelValue: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJGridTable_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJGridTable_GetView(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJGridTable_SendTableMessage(_obj: *mut ::libc::c_void,
                                         id: ::libc::c_int,
                                         val1: ::libc::c_int,
                                         val2: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn ELJLog_AddTraceMask(_obj: *mut ::libc::c_void,
                               str: *mut ::libc::c_void) -> ();
    pub fn ELJLog_Create(_obj: *mut ::libc::c_void, _fnc: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJLog_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJLog_DontCreateOnDemand(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJLog_EnableLogging(_obj: *mut ::libc::c_void,
                                doIt: ::libc::c_int) -> ::libc::c_int;
    pub fn ELJLog_Flush(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJLog_FlushActive(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJLog_GetActiveTarget() -> *mut ::libc::c_void;
    pub fn ELJLog_GetTimestamp(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJLog_GetTraceMask(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJLog_GetVerbose(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJLog_HasPendingMessages(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ELJLog_IsAllowedTraceMask(_obj: *mut ::libc::c_void,
                                     mask: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ELJLog_IsEnabled(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJLog_OnLog(_obj: *mut ::libc::c_void, level: ::libc::c_int,
                        szString: *mut ::libc::c_void, t: ::libc::c_int)
     -> ();
    pub fn ELJLog_RemoveTraceMask(_obj: *mut ::libc::c_void,
                                  str: *mut ::libc::c_void) -> ();
    pub fn ELJLog_Resume(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJLog_SetActiveTarget(pLogger: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJLog_SetTimestamp(_obj: *mut ::libc::c_void,
                               ts: *mut ::libc::c_void) -> ();
    pub fn ELJLog_SetTraceMask(_obj: *mut ::libc::c_void,
                               ulMask: ::libc::c_int) -> ();
    pub fn ELJLog_SetVerbose(_obj: *mut ::libc::c_void,
                             bVerbose: ::libc::c_int) -> ();
    pub fn ELJLog_Suspend(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMessageParameters_Create(_file: *mut ::libc::c_void,
                                      _type: *mut ::libc::c_void,
                                      _object: *mut ::libc::c_void,
                                      _func: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMessageParameters_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJPlotCurve_Create(_obj: *mut ::libc::c_void,
                               _str: *mut ::libc::c_void,
                               _end: *mut ::libc::c_void,
                               _y: *mut ::libc::c_void,
                               offsetY: ::libc::c_int,
                               startY: ::libc::c_double,
                               endY: ::libc::c_double) -> *mut ::libc::c_void;
    pub fn ELJPlotCurve_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJPlotCurve_GetEndY(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn ELJPlotCurve_GetOffsetY(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ELJPlotCurve_GetStartY(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn ELJPlotCurve_SetEndY(_obj: *mut ::libc::c_void,
                                endY: ::libc::c_double) -> ();
    pub fn ELJPlotCurve_SetOffsetY(_obj: *mut ::libc::c_void,
                                   offsetY: ::libc::c_int) -> ();
    pub fn ELJPlotCurve_SetPenNormal(_obj: *mut ::libc::c_void,
                                     pen: *mut ::libc::c_void) -> ();
    pub fn ELJPlotCurve_SetPenSelected(_obj: *mut ::libc::c_void,
                                       pen: *mut ::libc::c_void) -> ();
    pub fn ELJPlotCurve_SetStartY(_obj: *mut ::libc::c_void,
                                  startY: ::libc::c_double) -> ();
    pub fn ELJPreviewControlBar_Create(preview: *mut ::libc::c_void,
                                       buttons: ::libc::c_int,
                                       parent: *mut ::libc::c_void,
                                       title: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int,
                                       w: ::libc::c_int, h: ::libc::c_int,
                                       style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn ELJPreviewFrame_Create(_obj: *mut ::libc::c_void,
                                  _init: *mut ::libc::c_void,
                                  _create_canvas: *mut ::libc::c_void,
                                  _create_toolbar: *mut ::libc::c_void,
                                  preview: *mut ::libc::c_void,
                                  parent: *mut ::libc::c_void,
                                  title: *mut ::libc::c_void,
                                  x: ::libc::c_int, y: ::libc::c_int,
                                  w: ::libc::c_int, h: ::libc::c_int,
                                  style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn ELJPreviewFrame_GetControlBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJPreviewFrame_GetPreviewCanvas(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJPreviewFrame_GetPrintPreview(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJPreviewFrame_Initialize(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJPreviewFrame_SetControlBar(_obj: *mut ::libc::c_void,
                                         obj: *mut ::libc::c_void) -> ();
    pub fn ELJPreviewFrame_SetPreviewCanvas(_obj: *mut ::libc::c_void,
                                            obj: *mut ::libc::c_void) -> ();
    pub fn ELJPreviewFrame_SetPrintPreview(_obj: *mut ::libc::c_void,
                                           obj: *mut ::libc::c_void) -> ();
    pub fn ELJServer_Create(_eobj: *mut ::libc::c_void,
                            _cnct: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJServer_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJServer_Initialize(_obj: *mut ::libc::c_void,
                                name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ELJTextDropTarget_Create(_obj: *mut ::libc::c_void,
                                    _func: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn ELJTextDropTarget_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn ELJTextDropTarget_SetOnData(_obj: *mut ::libc::c_void,
                                       _func: *mut ::libc::c_void) -> ();
    pub fn ELJTextDropTarget_SetOnDragOver(_obj: *mut ::libc::c_void,
                                           _func: *mut ::libc::c_void) -> ();
    pub fn ELJTextDropTarget_SetOnDrop(_obj: *mut ::libc::c_void,
                                       _func: *mut ::libc::c_void) -> ();
    pub fn ELJTextDropTarget_SetOnEnter(_obj: *mut ::libc::c_void,
                                        _func: *mut ::libc::c_void) -> ();
    pub fn ELJTextDropTarget_SetOnLeave(_obj: *mut ::libc::c_void,
                                        _func: *mut ::libc::c_void) -> ();
    pub fn ELJTextValidator_Create(_obj: *mut ::libc::c_void,
                                   _fnc: *mut ::libc::c_void,
                                   _txt: *mut ::libc::c_void,
                                   _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbAntiflickerPlugin_Create(pPanel: *mut ::libc::c_void,
                                      paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbAntiflickerPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbAntiflickerPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbBarDragPlugin_Create(pPanel: *mut ::libc::c_void,
                                  paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbBarDragPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbBarDragPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbBarHintsPlugin_Create(pPanel: *mut ::libc::c_void,
                                   paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbBarHintsPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbBarHintsPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbBarHintsPlugin_SetGrooveCount(_obj: *mut ::libc::c_void,
                                           nGrooves: ::libc::c_int) -> ();
    pub fn cbBarInfo_Create() -> *mut ::libc::c_void;
    pub fn cbBarInfo_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbBarInfo_IsExpanded(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbBarInfo_IsFixed(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbBarSpy_Create(pPanel: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbBarSpy_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbBarSpy_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbBarSpy_ProcessEvent(_obj: *mut ::libc::c_void,
                                 event: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbBarSpy_SetBarWindow(_obj: *mut ::libc::c_void,
                                 pWnd: *mut ::libc::c_void) -> ();
    pub fn cbCloseBox_Create() -> *mut ::libc::c_void;
    pub fn cbCollapseBox_Create() -> *mut ::libc::c_void;
    pub fn cbCommonPaneProperties_Assign(_obj: *mut ::libc::c_void,
                                         _other: *mut ::libc::c_void) -> ();
    pub fn cbCommonPaneProperties_BarCollapseIconsOn(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_BarDragHintsOn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_BarFloatingOn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_ColProportionsOn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbCommonPaneProperties_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbCommonPaneProperties_ExactDockPredictionOn(_obj:
                                                            *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_MinCBarDim(_obj: *mut ::libc::c_void,
                                             _w: *mut ::libc::c_void,
                                             _h: *mut ::libc::c_void) -> ();
    pub fn cbCommonPaneProperties_NonDestructFrictionOn(_obj:
                                                            *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_OutOfPaneDragOn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_RealTimeUpdatesOn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_ResizeHandleSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_RowProportionsOn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCommonPaneProperties_SetBarCollapseIconsOn(_obj:
                                                            *mut ::libc::c_void,
                                                        _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetBarDragHintsOn(_obj: *mut ::libc::c_void,
                                                    _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetBarFloatingOn(_obj: *mut ::libc::c_void,
                                                   _val: ::libc::c_int) -> ();
    pub fn cbCommonPaneProperties_SetColProportionsOn(_obj:
                                                          *mut ::libc::c_void,
                                                      _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetExactDockPredictionOn(_obj:
                                                               *mut ::libc::c_void,
                                                           _val:
                                                               ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetMinCBarDim(_obj: *mut ::libc::c_void,
                                                _w: ::libc::c_int,
                                                _h: ::libc::c_int) -> ();
    pub fn cbCommonPaneProperties_SetNonDestructFrictionOn(_obj:
                                                               *mut ::libc::c_void,
                                                           _val:
                                                               ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetOutOfPaneDragOn(_obj:
                                                         *mut ::libc::c_void,
                                                     _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetRealTimeUpdatesOn(_obj:
                                                           *mut ::libc::c_void,
                                                       _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetResizeHandleSize(_obj:
                                                          *mut ::libc::c_void,
                                                      _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetRowProportionsOn(_obj:
                                                          *mut ::libc::c_void,
                                                      _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_SetShow3DPaneBorderOn(_obj:
                                                            *mut ::libc::c_void,
                                                        _val: ::libc::c_int)
     -> ();
    pub fn cbCommonPaneProperties_Show3DPaneBorderOn(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbCustomizeBarEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbCustomizeBarEvent_ClickPos(_obj: *mut ::libc::c_void,
                                        _x: *mut ::libc::c_void,
                                        _y: *mut ::libc::c_void) -> ();
    pub fn cbCustomizeLayoutEvent_ClickPos(_obj: *mut ::libc::c_void,
                                           _x: *mut ::libc::c_void,
                                           _y: *mut ::libc::c_void) -> ();
    pub fn cbDimInfo_Assign(_obj: *mut ::libc::c_void,
                            other: *mut ::libc::c_void) -> ();
    pub fn cbDimInfo_Create(x: ::libc::c_int, y: ::libc::c_int,
                            isFixed: ::libc::c_int, gap: ::libc::c_int,
                            pDimHandler: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDimInfo_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbDimInfo_CreateWithHandler(pDimHandler: *mut ::libc::c_void,
                                       isFixed: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbDimInfo_CreateWithInfo(dh_x: ::libc::c_int, dh_y: ::libc::c_int,
                                    dv_x: ::libc::c_int, dv_y: ::libc::c_int,
                                    f_x: ::libc::c_int, f_y: ::libc::c_int,
                                    isFixed: ::libc::c_int,
                                    horizGap: ::libc::c_int,
                                    vertGap: ::libc::c_int,
                                    pDimHandler: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDimInfo_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbDimInfo_GetDimHandler(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDockBox_Create() -> *mut ::libc::c_void;
    pub fn cbDockPane_BarPresent(_obj: *mut ::libc::c_void,
                                 pBar: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbDockPane_Create(alignment: ::libc::c_int,
                             pPanel: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDockPane_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbDockPane_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_GetAlignment(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDockPane_GetBarInfoByWindow(_obj: *mut ::libc::c_void,
                                         pBarWnd: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDockPane_GetBarResizeRange(_obj: *mut ::libc::c_void,
                                        pBar: *mut ::libc::c_void,
                                        from: *mut ::libc::c_void,
                                        till: *mut ::libc::c_void,
                                        forLeftHandle: ::libc::c_int) -> ();
    pub fn cbDockPane_GetDockingState(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDockPane_GetFirstRow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDockPane_GetPaneHeight(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDockPane_GetRealRect(_obj: *mut ::libc::c_void,
                                  _x: *mut ::libc::c_void,
                                  _y: *mut ::libc::c_void,
                                  _w: *mut ::libc::c_void,
                                  _h: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_GetRowList(_obj: *mut ::libc::c_void,
                                 _ref: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbDockPane_GetRowResizeRange(_obj: *mut ::libc::c_void,
                                        pRow: *mut ::libc::c_void,
                                        from: *mut ::libc::c_void,
                                        till: *mut ::libc::c_void,
                                        forUpperHandle: ::libc::c_int) -> ();
    pub fn cbDockPane_HitTestPaneItems(_obj: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int,
                                       ppRow: *mut ::libc::c_void,
                                       ppBar: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDockPane_InsertBarByCoord(_obj: *mut ::libc::c_void,
                                       pBar: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int,
                                       w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn cbDockPane_InsertBarByInfo(_obj: *mut ::libc::c_void,
                                      pBarInfo: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_InsertBarToRow(_obj: *mut ::libc::c_void,
                                     pBar: *mut ::libc::c_void,
                                     pIntoRow: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_InsertRow(_obj: *mut ::libc::c_void,
                                pRow: *mut ::libc::c_void,
                                pBeforeRow: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_IsHorizontal(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDockPane_MatchesMask(_obj: *mut ::libc::c_void,
                                  paneMask: ::libc::c_int) -> ::libc::c_int;
    pub fn cbDockPane_RemoveBar(_obj: *mut ::libc::c_void,
                                pBar: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_RemoveRow(_obj: *mut ::libc::c_void,
                                pRow: *mut ::libc::c_void) -> ();
    pub fn cbDockPane_SetBoundsInParent(_obj: *mut ::libc::c_void,
                                        x: ::libc::c_int, y: ::libc::c_int,
                                        w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn cbDockPane_SetMargins(_obj: *mut ::libc::c_void,
                                 top: ::libc::c_int, bottom: ::libc::c_int,
                                 left: ::libc::c_int, right: ::libc::c_int)
     -> ();
    pub fn cbDockPane_SetPaneWidth(_obj: *mut ::libc::c_void,
                                   width: ::libc::c_int) -> ();
    pub fn cbDrawBarDecorEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawBarDecorEvent_BoundsInParent(_obj: *mut ::libc::c_void,
                                              _x: *mut ::libc::c_void,
                                              _y: *mut ::libc::c_void,
                                              _w: *mut ::libc::c_void,
                                              _h: *mut ::libc::c_void) -> ();
    pub fn cbDrawBarDecorEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawBarHandlesEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawBarHandlesEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawHintRectEvent_EraseRect(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDrawHintRectEvent_IsInClient(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDrawHintRectEvent_LastTime(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbDrawHintRectEvent_Rect(_obj: *mut ::libc::c_void,
                                    _x: *mut ::libc::c_void,
                                    _y: *mut ::libc::c_void,
                                    _w: *mut ::libc::c_void,
                                    _h: *mut ::libc::c_void) -> ();
    pub fn cbDrawPaneBkGroundEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawPaneDecorEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawRowBkGroundEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawRowBkGroundEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawRowDecorEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawRowDecorEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawRowHandlesEvent_Dc(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDrawRowHandlesEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbDynToolBarDimHandler_Create() -> *mut ::libc::c_void;
    pub fn cbDynToolBarDimHandler_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbFinishDrawInAreaEvent_Area(_obj: *mut ::libc::c_void,
                                        _x: *mut ::libc::c_void,
                                        _y: *mut ::libc::c_void,
                                        _w: *mut ::libc::c_void,
                                        _h: *mut ::libc::c_void) -> ();
    pub fn cbFloatedBarWindow_Create(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbFloatedBarWindow_GetBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbFloatedBarWindow_PositionFloatedWnd(_obj: *mut ::libc::c_void,
                                                 _x: ::libc::c_int,
                                                 _y: ::libc::c_int,
                                                 _w: ::libc::c_int,
                                                 _h: ::libc::c_int) -> ();
    pub fn cbFloatedBarWindow_SetBar(_obj: *mut ::libc::c_void,
                                     _bar: *mut ::libc::c_void) -> ();
    pub fn cbFloatedBarWindow_SetLayout(_obj: *mut ::libc::c_void,
                                        _layout: *mut ::libc::c_void) -> ();
    pub fn cbGCUpdatesMgr_Create(pPanel: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbGCUpdatesMgr_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbGCUpdatesMgr_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbGCUpdatesMgr_UpdateNow(_obj: *mut ::libc::c_void) -> ();
    pub fn cbHintAnimationPlugin_Create(pPanel: *mut ::libc::c_void,
                                        paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbHintAnimationPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbHintAnimationPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbInsertBarEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbInsertBarEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbLayoutRowEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbLeftDClickEvent_Pos(_obj: *mut ::libc::c_void,
                                 _x: *mut ::libc::c_void,
                                 _y: *mut ::libc::c_void) -> ();
    pub fn cbLeftDownEvent_Pos(_obj: *mut ::libc::c_void,
                               _x: *mut ::libc::c_void,
                               _y: *mut ::libc::c_void) -> ();
    pub fn cbLeftUpEvent_Pos(_obj: *mut ::libc::c_void,
                             _x: *mut ::libc::c_void, _y: *mut ::libc::c_void)
     -> ();
    pub fn cbMiniButton_Create() -> *mut ::libc::c_void;
    pub fn cbMiniButton_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbMiniButton_Dim(_obj: *mut ::libc::c_void,
                            _w: *mut ::libc::c_void, _h: *mut ::libc::c_void)
     -> ();
    pub fn cbMiniButton_DragStarted(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbMiniButton_Enable(_obj: *mut ::libc::c_void,
                               enable: ::libc::c_int) -> ();
    pub fn cbMiniButton_Enabled(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbMiniButton_HitTest(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int) -> ::libc::c_int;
    pub fn cbMiniButton_IsPressed(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbMiniButton_Layout(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbMiniButton_Pane(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbMiniButton_Plugin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbMiniButton_Pos(_obj: *mut ::libc::c_void,
                            _x: *mut ::libc::c_void, _y: *mut ::libc::c_void)
     -> ();
    pub fn cbMiniButton_Pressed(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbMiniButton_Refresh(_obj: *mut ::libc::c_void) -> ();
    pub fn cbMiniButton_Reset(_obj: *mut ::libc::c_void) -> ();
    pub fn cbMiniButton_SetPos(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                               y: ::libc::c_int) -> ();
    pub fn cbMiniButton_Visible(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbMiniButton_WasClicked(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbMiniButton_Wnd(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn cbMotionEvent_Pos(_obj: *mut ::libc::c_void,
                             _x: *mut ::libc::c_void, _y: *mut ::libc::c_void)
     -> ();
    pub fn cbPaneDrawPlugin_Create(pPanel: *mut ::libc::c_void,
                                   paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbPaneDrawPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbPaneDrawPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbPluginBase_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbPluginBase_GetPaneMask(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbPluginBase_IsReady(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cbPluginBase_Plugin(_swt: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn cbPluginBase_ProcessEvent(_obj: *mut ::libc::c_void,
                                     event: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbPluginEvent_Pane(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbRemoveBarEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbResizeBarEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbResizeBarEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbResizeRowEvent_ForUpperHandle(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbResizeRowEvent_HandleOfs(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cbResizeRowEvent_Row(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbRightDownEvent_Pos(_obj: *mut ::libc::c_void,
                                _x: *mut ::libc::c_void,
                                _y: *mut ::libc::c_void) -> ();
    pub fn cbRightUpEvent_Pos(_obj: *mut ::libc::c_void,
                              _x: *mut ::libc::c_void,
                              _y: *mut ::libc::c_void) -> ();
    pub fn cbRowDragPlugin_Create(pPanel: *mut ::libc::c_void,
                                  paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbRowDragPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbRowDragPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbRowInfo_Create() -> *mut ::libc::c_void;
    pub fn cbRowInfo_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbRowInfo_GetFirstBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbRowLayoutPlugin_Create(pPanel: *mut ::libc::c_void,
                                    paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbRowLayoutPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbRowLayoutPlugin_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn cbSimpleCustomizationPlugin_Create(pPanel: *mut ::libc::c_void,
                                              paneMask: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn cbSimpleCustomizationPlugin_CreateDefault() -> *mut ::libc::c_void;
    pub fn cbSimpleCustomizationPlugin_Delete(_obj: *mut ::libc::c_void)
     -> ();
    pub fn cbSizeBarWndEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbSizeBarWndEvent_BoundsInParent(_obj: *mut ::libc::c_void,
                                            _x: *mut ::libc::c_void,
                                            _y: *mut ::libc::c_void,
                                            _w: *mut ::libc::c_void,
                                            _h: *mut ::libc::c_void) -> ();
    pub fn cbStartBarDraggingEvent_Bar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cbStartBarDraggingEvent_Pos(_obj: *mut ::libc::c_void,
                                       _x: *mut ::libc::c_void,
                                       _y: *mut ::libc::c_void) -> ();
    pub fn cbStartDrawInAreaEvent_Area(_obj: *mut ::libc::c_void,
                                       _x: *mut ::libc::c_void,
                                       _y: *mut ::libc::c_void,
                                       _w: *mut ::libc::c_void,
                                       _h: *mut ::libc::c_void) -> ();
    pub fn wxAcceleratorEntry_Create(flags: ::libc::c_int,
                                     keyCode: ::libc::c_int,
                                     cmd: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxAcceleratorEntry_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxAcceleratorEntry_GetCommand(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxAcceleratorEntry_GetFlags(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxAcceleratorEntry_GetKeyCode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxAcceleratorEntry_Set(_obj: *mut ::libc::c_void,
                                  flags: ::libc::c_int,
                                  keyCode: ::libc::c_int, cmd: ::libc::c_int)
     -> ();
    pub fn wxAcceleratorTable_Create(n: ::libc::c_int,
                                     entries: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxAcceleratorTable_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxActivateEvent_CopyObject(_obj: *mut ::libc::c_void,
                                      obj: *mut ::libc::c_void) -> ();
    pub fn wxActivateEvent_GetActive(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn PopProvider() -> ::libc::c_int;
    pub fn PushProvider(provider: *mut ::libc::c_void) -> ();
    pub fn RemoveProvider(provider: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxAutoBufferedPaintDC_Create(window: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxAutoBufferedPaintDC_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_AddHandler(handler: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_CleanUpHandlers() -> ();
    pub fn wxBitmap_Create(_data: *mut ::libc::c_void, _type: ::libc::c_int,
                           _width: ::libc::c_int, _height: ::libc::c_int,
                           _depth: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBitmap_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxBitmap_CreateEmpty(_width: ::libc::c_int, _height: ::libc::c_int,
                                _depth: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBitmap_CreateFromXPM(data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBitmap_CreateLoad(name: *mut ::libc::c_void,
                               _type: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBitmap_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_FindHandlerByExtension(extension: *mut ::libc::c_void,
                                           _type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBitmap_FindHandlerByName(name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBitmap_FindHandlerByType(_type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBitmap_GetDepth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBitmap_GetHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBitmap_GetMask(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxBitmap_GetSubBitmap(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int, w: ::libc::c_int,
                                 h: ::libc::c_int, _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxBitmap_GetWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBitmap_InitStandardHandlers() -> ();
    pub fn wxBitmap_InsertHandler(handler: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_LoadFile(_obj: *mut ::libc::c_void,
                             name: *mut ::libc::c_void, _type: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxBitmap_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBitmap_RemoveHandler(name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBitmap_SaveFile(_obj: *mut ::libc::c_void,
                             name: *mut ::libc::c_void, _type: ::libc::c_int,
                             cmap: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBitmap_SetDepth(_obj: *mut ::libc::c_void, d: ::libc::c_int)
     -> ();
    pub fn wxBitmap_SetHeight(_obj: *mut ::libc::c_void, h: ::libc::c_int)
     -> ();
    pub fn wxBitmap_SetMask(_obj: *mut ::libc::c_void,
                            mask: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_SetWidth(_obj: *mut ::libc::c_void, w: ::libc::c_int)
     -> ();
    pub fn wxBitmapButton_Create(_prt: *mut ::libc::c_void,
                                 _id: ::libc::c_int,
                                 _bmp: *mut ::libc::c_void,
                                 _lft: ::libc::c_int, _top: ::libc::c_int,
                                 _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                 _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBitmapButton_GetBitmapDisabled(_obj: *mut ::libc::c_void,
                                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_GetBitmapFocus(_obj: *mut ::libc::c_void,
                                         _ref: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_GetBitmapLabel(_obj: *mut ::libc::c_void,
                                         _ref: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_GetBitmapSelected(_obj: *mut ::libc::c_void,
                                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_GetMarginX(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxBitmapButton_GetMarginY(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxBitmapButton_SetBitmapDisabled(_obj: *mut ::libc::c_void,
                                            disabled: *mut ::libc::c_void)
     -> ();
    pub fn wxBitmapButton_SetBitmapFocus(_obj: *mut ::libc::c_void,
                                         focus: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_SetBitmapLabel(_obj: *mut ::libc::c_void,
                                         bitmap: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_SetBitmapSelected(_obj: *mut ::libc::c_void,
                                            sel: *mut ::libc::c_void) -> ();
    pub fn wxBitmapButton_SetMargins(_obj: *mut ::libc::c_void,
                                     x: ::libc::c_int, y: ::libc::c_int)
     -> ();
    pub fn wxBitmapToggleButton_Create(parent: *mut ::libc::c_void,
                                       id: ::libc::c_int,
                                       _bmp: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int,
                                       w: ::libc::c_int, h: ::libc::c_int,
                                       style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBitmapToggleButton_Enable(_obj: *mut ::libc::c_void,
                                       enable: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxBitmapToggleButton_GetValue(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxBitmapToggleButton_SetValue(_obj: *mut ::libc::c_void,
                                         state: ::libc::c_int) -> ();
    pub fn wxBitmapToggleButton_SetBitmapLabel(_obj: *mut ::libc::c_void,
                                               _bmp: *mut ::libc::c_void)
     -> ();
    pub fn BitmapDataObject_Create(_bmp: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn BitmapDataObject_CreateEmpty() -> *mut ::libc::c_void;
    pub fn BitmapDataObject_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn BitmapDataObject_GetBitmap(_obj: *mut ::libc::c_void,
                                      _bmp: *mut ::libc::c_void) -> ();
    pub fn BitmapDataObject_SetBitmap(_obj: *mut ::libc::c_void,
                                      _bmp: *mut ::libc::c_void) -> ();
    pub fn wxBoxSizer_CalcMin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBoxSizer_Create(orient: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBoxSizer_GetOrientation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxBoxSizer_RecalcSizes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxBrush_Assign(_obj: *mut ::libc::c_void,
                          brush: *mut ::libc::c_void) -> ();
    pub fn wxBrush_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxBrush_CreateFromBitmap(bitmap: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBrush_CreateFromColour(col: *mut ::libc::c_void,
                                    style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBrush_CreateFromStock(id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBrush_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxBrush_GetColour(_obj: *mut ::libc::c_void,
                             _ref: *mut ::libc::c_void) -> ();
    pub fn wxBrush_GetStipple(_obj: *mut ::libc::c_void,
                              _ref: *mut ::libc::c_void) -> ();
    pub fn wxBrush_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBrush_IsEqual(_obj: *mut ::libc::c_void,
                           brush: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBrush_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBrush_SetColour(_obj: *mut ::libc::c_void,
                             col: *mut ::libc::c_void) -> ();
    pub fn wxBrush_SetColourSingle(_obj: *mut ::libc::c_void,
                                   r: ::libc::c_char, g: ::libc::c_char,
                                   b: ::libc::c_char) -> ();
    pub fn wxBrush_SetStipple(_obj: *mut ::libc::c_void,
                              stipple: *mut ::libc::c_void) -> ();
    pub fn wxBrush_SetStyle(_obj: *mut ::libc::c_void, style: ::libc::c_int)
     -> ();
    pub fn wxBufferedDC_CreateByDCAndSize(dc: *mut ::libc::c_void,
                                          width: ::libc::c_int,
                                          hight: ::libc::c_int,
                                          style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBufferedDC_CreateByDCAndBitmap(dc: *mut ::libc::c_void,
                                            bitmap: *mut ::libc::c_void,
                                            style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBufferedDC_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxBufferedPaintDC_Create(window: *mut ::libc::c_void,
                                    style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBufferedPaintDC_CreateWithBitmap(window: *mut ::libc::c_void,
                                              bitmap: *mut ::libc::c_void,
                                              style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxBufferedPaintDC_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxBusyCursor_Create() -> *mut ::libc::c_void;
    pub fn wxBusyCursor_CreateWithCursor(_cur: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBusyCursor_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxBusyInfo_Create(_txt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBusyInfo_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxButton_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                           _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                           _top: ::libc::c_int, _wdt: ::libc::c_int,
                           _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxButton_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                        colour: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxButton_SetDefault(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCalculateLayoutEvent_Create(id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxCalculateLayoutEvent_GetFlags(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalculateLayoutEvent_GetRect(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCalculateLayoutEvent_SetFlags(_obj: *mut ::libc::c_void,
                                           flags: ::libc::c_int) -> ();
    pub fn wxCalculateLayoutEvent_SetRect(_obj: *mut ::libc::c_void,
                                          x: ::libc::c_int, y: ::libc::c_int,
                                          w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxCalendarCtrl_Create(_prt: *mut ::libc::c_void,
                                 _id: ::libc::c_int,
                                 _dat: *mut ::libc::c_void,
                                 _lft: ::libc::c_int, _top: ::libc::c_int,
                                 _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                 _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxCalendarCtrl_EnableHolidayDisplay(_obj: *mut ::libc::c_void,
                                               display: ::libc::c_int) -> ();
    pub fn wxCalendarCtrl_EnableMonthChange(_obj: *mut ::libc::c_void,
                                            enable: ::libc::c_int) -> ();
    pub fn wxCalendarCtrl_GetAttr(_obj: *mut ::libc::c_void,
                                  day: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxCalendarCtrl_GetDate(_obj: *mut ::libc::c_void,
                                  date: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_GetHeaderColourBg(_obj: *mut ::libc::c_void,
                                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_GetHeaderColourFg(_obj: *mut ::libc::c_void,
                                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_GetHighlightColourBg(_obj: *mut ::libc::c_void,
                                               _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxCalendarCtrl_GetHighlightColourFg(_obj: *mut ::libc::c_void,
                                               _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxCalendarCtrl_GetHolidayColourBg(_obj: *mut ::libc::c_void,
                                             _ref: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_GetHolidayColourFg(_obj: *mut ::libc::c_void,
                                             _ref: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_HitTest(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                  y: ::libc::c_int, date: *mut ::libc::c_void,
                                  wd: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCalendarCtrl_ResetAttr(_obj: *mut ::libc::c_void,
                                    day: ::libc::c_int) -> ();
    pub fn wxCalendarCtrl_SetAttr(_obj: *mut ::libc::c_void,
                                  day: ::libc::c_int,
                                  attr: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_SetDate(_obj: *mut ::libc::c_void,
                                  date: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_SetHeaderColours(_obj: *mut ::libc::c_void,
                                           colFg: *mut ::libc::c_void,
                                           colBg: *mut ::libc::c_void) -> ();
    pub fn wxCalendarCtrl_SetHighlightColours(_obj: *mut ::libc::c_void,
                                              colFg: *mut ::libc::c_void,
                                              colBg: *mut ::libc::c_void)
     -> ();
    pub fn wxCalendarCtrl_SetHoliday(_obj: *mut ::libc::c_void,
                                     day: ::libc::c_int) -> ();
    pub fn wxCalendarCtrl_SetHolidayColours(_obj: *mut ::libc::c_void,
                                            colFg: *mut ::libc::c_void,
                                            colBg: *mut ::libc::c_void) -> ();
    pub fn wxCalendarDateAttr_Create(_ctxt: *mut ::libc::c_void,
                                     _cbck: *mut ::libc::c_void,
                                     _cbrd: *mut ::libc::c_void,
                                     _fnt: *mut ::libc::c_void,
                                     _brd: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxCalendarDateAttr_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxCalendarDateAttr_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCalendarDateAttr_GetBackgroundColour(_obj: *mut ::libc::c_void,
                                                  _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxCalendarDateAttr_GetBorder(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_GetBorderColour(_obj: *mut ::libc::c_void,
                                              _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxCalendarDateAttr_GetFont(_obj: *mut ::libc::c_void,
                                      _ref: *mut ::libc::c_void) -> ();
    pub fn wxCalendarDateAttr_GetTextColour(_obj: *mut ::libc::c_void,
                                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxCalendarDateAttr_HasBackgroundColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_HasBorder(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_HasBorderColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_HasFont(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_HasTextColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_IsHoliday(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCalendarDateAttr_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                                  col: *mut ::libc::c_void)
     -> ();
    pub fn wxCalendarDateAttr_SetBorder(_obj: *mut ::libc::c_void,
                                        border: ::libc::c_int) -> ();
    pub fn wxCalendarDateAttr_SetBorderColour(_obj: *mut ::libc::c_void,
                                              col: *mut ::libc::c_void) -> ();
    pub fn wxCalendarDateAttr_SetFont(_obj: *mut ::libc::c_void,
                                      font: *mut ::libc::c_void) -> ();
    pub fn wxCalendarDateAttr_SetHoliday(_obj: *mut ::libc::c_void,
                                         holiday: ::libc::c_int) -> ();
    pub fn wxCalendarDateAttr_SetTextColour(_obj: *mut ::libc::c_void,
                                            col: *mut ::libc::c_void) -> ();
    pub fn wxCalendarEvent_GetDate(_obj: *mut ::libc::c_void,
                                   _dte: *mut ::libc::c_void) -> ();
    pub fn wxCalendarEvent_GetWeekDay(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCaret_Create(_wnd: *mut ::libc::c_void, _wth: ::libc::c_int,
                          _hgt: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxCaret_GetBlinkTime() -> ::libc::c_int;
    pub fn wxCaret_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCaret_GetSize(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxCaret_GetWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCaret_Hide(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCaret_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCaret_IsVisible(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCaret_Move(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                        y: ::libc::c_int) -> ();
    pub fn wxCaret_SetBlinkTime(milliseconds: ::libc::c_int) -> ();
    pub fn wxCaret_SetSize(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                           height: ::libc::c_int) -> ();
    pub fn wxCaret_Show(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCheckBox_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                             _top: ::libc::c_int, _wdt: ::libc::c_int,
                             _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxCheckBox_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCheckBox_GetValue(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCheckBox_SetValue(_obj: *mut ::libc::c_void,
                               value: ::libc::c_int) -> ();
    pub fn wxCheckListBox_Check(_obj: *mut ::libc::c_void,
                                item: ::libc::c_int, check: ::libc::c_int)
     -> ();
    pub fn wxCheckListBox_Create(_prt: *mut ::libc::c_void,
                                 _id: ::libc::c_int, _lft: ::libc::c_int,
                                 _top: ::libc::c_int, _wdt: ::libc::c_int,
                                 _hgt: ::libc::c_int, n: ::libc::c_int,
                                 str: *mut *mut ::libc::c_char,
                                 _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxCheckListBox_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCheckListBox_IsChecked(_obj: *mut ::libc::c_void,
                                    item: ::libc::c_int) -> ::libc::c_int;
    pub fn wxChoice_Append(_obj: *mut ::libc::c_void,
                           item: *mut ::libc::c_void) -> ();
    pub fn wxChoice_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxChoice_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                           _lft: ::libc::c_int, _top: ::libc::c_int,
                           _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                           n: ::libc::c_int, str: *mut *mut ::libc::c_char,
                           _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxChoice_Delete(_obj: *mut ::libc::c_void, n: ::libc::c_int) -> ();
    pub fn wxChoice_FindString(_obj: *mut ::libc::c_void,
                               s: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxChoice_GetCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxChoice_GetSelection(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxChoice_GetString(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxChoice_SetSelection(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> ();
    pub fn wxChoice_SetString(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                              s: *mut ::libc::c_void) -> ();
    pub fn wxClassInfo_CreateClassByName(_inf: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClassInfo_GetClassName(_inf: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClassInfo_IsKindOf(_obj: *mut ::libc::c_void,
                                _name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClientDC_Create(win: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxClientDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxClipboard_AddData(_obj: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClipboard_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxClipboard_Close(_obj: *mut ::libc::c_void) -> ();
    pub fn wxClipboard_Create() -> *mut ::libc::c_void;
    pub fn wxClipboard_Flush(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClipboard_GetData(_obj: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClipboard_IsOpened(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClipboard_IsSupported(_obj: *mut ::libc::c_void,
                                   format: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxClipboard_Open(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClipboard_SetData(_obj: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClipboard_UsePrimarySelection(_obj: *mut ::libc::c_void,
                                           primary: ::libc::c_int) -> ();
    pub fn wxCloseEvent_CanVeto(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCloseEvent_CopyObject(_obj: *mut ::libc::c_void,
                                   obj: *mut ::libc::c_void) -> ();
    pub fn wxCloseEvent_GetLoggingOff(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCloseEvent_GetVeto(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCloseEvent_SetCanVeto(_obj: *mut ::libc::c_void,
                                   canVeto: ::libc::c_int) -> ();
    pub fn wxCloseEvent_SetLoggingOff(_obj: *mut ::libc::c_void,
                                      logOff: ::libc::c_int) -> ();
    pub fn wxCloseEvent_Veto(_obj: *mut ::libc::c_void, veto: ::libc::c_int)
     -> ();
    pub fn wxColour_Alpha(_obj: *mut ::libc::c_void) -> uint8_t;
    pub fn wxColour_Assign(_obj: *mut ::libc::c_void,
                           other: *mut ::libc::c_void) -> ();
    pub fn wxColour_Blue(_obj: *mut ::libc::c_void) -> uint8_t;
    pub fn wxColour_Copy(_obj: *mut ::libc::c_void,
                         _other: *mut ::libc::c_void) -> ();
    pub fn wxColour_CreateByName(_name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxColour_CreateEmpty() -> *mut ::libc::c_void;
    pub fn wxColour_CreateFromStock(id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxColour_CreateRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t,
                              _alpha: uint8_t) -> *mut ::libc::c_void;
    pub fn wxColour_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxColour_Green(_obj: *mut ::libc::c_void) -> uint8_t;
    pub fn wxColour_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxColour_Red(_obj: *mut ::libc::c_void) -> uint8_t;
    pub fn wxColour_Set(_obj: *mut ::libc::c_void, _red: uint8_t,
                        _green: uint8_t, _blue: uint8_t, _alpha: uint8_t)
     -> ();
    pub fn wxColour_SetByName(_obj: *mut ::libc::c_void,
                              _name: *mut ::libc::c_void) -> ();
    pub fn wxColour_ValidName(_name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxColourData_Create() -> *mut ::libc::c_void;
    pub fn wxColourData_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxColourData_GetChooseFull(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxColourData_GetColour(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxColourData_GetCustomColour(_obj: *mut ::libc::c_void,
                                        i: ::libc::c_int,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxColourData_SetChooseFull(_obj: *mut ::libc::c_void,
                                      flag: ::libc::c_int) -> ();
    pub fn wxColourData_SetColour(_obj: *mut ::libc::c_void,
                                  colour: *mut ::libc::c_void) -> ();
    pub fn wxColourData_SetCustomColour(_obj: *mut ::libc::c_void,
                                        i: ::libc::c_int,
                                        colour: *mut ::libc::c_void) -> ();
    pub fn wxColourDialog_Create(_prt: *mut ::libc::c_void,
                                 col: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxColourDialog_GetColourData(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_Append(_obj: *mut ::libc::c_void,
                             item: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_AppendData(_obj: *mut ::libc::c_void,
                                 item: *mut ::libc::c_void,
                                 d: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_Copy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                             _top: ::libc::c_int, _wdt: ::libc::c_int,
                             _hgt: ::libc::c_int, n: ::libc::c_int,
                             str: *mut *mut ::libc::c_char,
                             _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxComboBox_Cut(_obj: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_Delete(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> ();
    pub fn wxComboBox_FindString(_obj: *mut ::libc::c_void,
                                 s: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxComboBox_GetClientData(_obj: *mut ::libc::c_void,
                                    n: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxComboBox_GetCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxComboBox_GetInsertionPoint(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxComboBox_GetLastPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxComboBox_GetSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxComboBox_GetString(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxComboBox_GetStringSelection(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxComboBox_GetValue(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxComboBox_Paste(_obj: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_Remove(_obj: *mut ::libc::c_void, from: ::libc::c_int,
                             to: ::libc::c_int) -> ();
    pub fn wxComboBox_Replace(_obj: *mut ::libc::c_void, from: ::libc::c_int,
                              to: ::libc::c_int, value: *mut ::libc::c_void)
     -> ();
    pub fn wxComboBox_SetClientData(_obj: *mut ::libc::c_void,
                                    n: ::libc::c_int,
                                    clientData: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_SetEditable(_obj: *mut ::libc::c_void,
                                  editable: ::libc::c_int) -> ();
    pub fn wxComboBox_SetInsertionPoint(_obj: *mut ::libc::c_void,
                                        pos: ::libc::c_int) -> ();
    pub fn wxComboBox_SetInsertionPointEnd(_obj: *mut ::libc::c_void) -> ();
    pub fn wxComboBox_SetSelection(_obj: *mut ::libc::c_void,
                                   n: ::libc::c_int) -> ();
    pub fn wxComboBox_SetTextSelection(_obj: *mut ::libc::c_void,
                                       from: ::libc::c_int, to: ::libc::c_int)
     -> ();
    pub fn wxCommandEvent_CopyObject(_obj: *mut ::libc::c_void,
                                     object_dest: *mut ::libc::c_void) -> ();
    pub fn wxCommandEvent_Create(_typ: ::libc::c_int, _id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxCommandEvent_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCommandEvent_GetClientData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCommandEvent_GetClientObject(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCommandEvent_GetExtraLong(_obj: *mut ::libc::c_void)
     -> ::libc::c_long;
    pub fn wxCommandEvent_GetInt(_obj: *mut ::libc::c_void) -> ::libc::c_long;
    pub fn wxCommandEvent_GetSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandEvent_GetString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCommandEvent_IsChecked(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandEvent_IsSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandEvent_SetClientData(_obj: *mut ::libc::c_void,
                                        clientData: *mut ::libc::c_void)
     -> ();
    pub fn wxCommandEvent_SetClientObject(_obj: *mut ::libc::c_void,
                                          clientObject: *mut ::libc::c_void)
     -> ();
    pub fn wxCommandEvent_SetExtraLong(_obj: *mut ::libc::c_void,
                                       extraLong: ::libc::c_long) -> ();
    pub fn wxCommandEvent_SetInt(_obj: *mut ::libc::c_void, i: ::libc::c_int)
     -> ();
    pub fn wxCommandEvent_SetString(_obj: *mut ::libc::c_void,
                                    s: *mut ::libc::c_void) -> ();
    pub fn wxCommandProcessor_CanRedo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandProcessor_CanUndo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandProcessor_ClearCommands(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCommandProcessor_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCommandProcessor_GetCommands(_obj: *mut ::libc::c_void,
                                          _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandProcessor_GetEditMenu(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCommandProcessor_GetMaxCommands(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandProcessor_Initialize(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCommandProcessor_Redo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandProcessor_SetEditMenu(_obj: *mut ::libc::c_void,
                                          menu: *mut ::libc::c_void) -> ();
    pub fn wxCommandProcessor_SetMenuStrings(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCommandProcessor_Submit(_obj: *mut ::libc::c_void,
                                     command: *mut ::libc::c_void,
                                     storeIt: ::libc::c_int) -> ::libc::c_int;
    pub fn wxCommandProcessor_Undo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxCommandProcessor_wxCommandProcessor(maxCommands: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxCondition_Broadcast(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCondition_Create(_mut: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxCondition_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCondition_Signal(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCondition_Wait(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCondition_WaitFor(_obj: *mut ::libc::c_void, sec: ::libc::c_int,
                               nsec: ::libc::c_int) -> ::libc::c_int;
    pub fn wxConfigBase_Create() -> *mut ::libc::c_void;
    pub fn wxConfigBase_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxConfigBase_DeleteAll(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxConfigBase_DeleteEntry(_obj: *mut ::libc::c_void,
                                    key: *mut ::libc::c_void,
                                    bDeleteGroupIfEmpty: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxConfigBase_DeleteGroup(_obj: *mut ::libc::c_void,
                                    key: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_Exists(_obj: *mut ::libc::c_void,
                               strName: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxConfigBase_ExpandEnvVars(_obj: *mut ::libc::c_void,
                                      str: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_Flush(_obj: *mut ::libc::c_void,
                              bCurrentOnly: ::libc::c_int) -> ::libc::c_int;
    pub fn wxConfigBase_GetAppName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_GetEntryType(_obj: *mut ::libc::c_void,
                                     name: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_GetFirstEntry(_obj: *mut ::libc::c_void,
                                      lIndex: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_GetFirstGroup(_obj: *mut ::libc::c_void,
                                      lIndex: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_GetNextEntry(_obj: *mut ::libc::c_void,
                                     lIndex: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_GetNextGroup(_obj: *mut ::libc::c_void,
                                     lIndex: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_GetNumberOfEntries(_obj: *mut ::libc::c_void,
                                           bRecursive: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxConfigBase_GetNumberOfGroups(_obj: *mut ::libc::c_void,
                                          bRecursive: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxConfigBase_GetPath(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxConfigBase_GetVendorName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_HasEntry(_obj: *mut ::libc::c_void,
                                 strName: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_HasGroup(_obj: *mut ::libc::c_void,
                                 strName: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_IsExpandingEnvVars(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_IsRecordingDefaults(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_ReadBool(_obj: *mut ::libc::c_void,
                                 key: *mut ::libc::c_void,
                                 defVal: ::libc::c_int) -> ::libc::c_int;
    pub fn wxConfigBase_ReadDouble(_obj: *mut ::libc::c_void,
                                   key: *mut ::libc::c_void,
                                   defVal: ::libc::c_double)
     -> ::libc::c_double;
    pub fn wxConfigBase_ReadInteger(_obj: *mut ::libc::c_void,
                                    key: *mut ::libc::c_void,
                                    defVal: ::libc::c_int) -> ::libc::c_int;
    pub fn wxConfigBase_ReadString(_obj: *mut ::libc::c_void,
                                   key: *mut ::libc::c_void,
                                   defVal: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxConfigBase_RenameEntry(_obj: *mut ::libc::c_void,
                                    oldName: *mut ::libc::c_void,
                                    newName: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_RenameGroup(_obj: *mut ::libc::c_void,
                                    oldName: *mut ::libc::c_void,
                                    newName: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxConfigBase_SetAppName(_obj: *mut ::libc::c_void,
                                   appName: *mut ::libc::c_void) -> ();
    pub fn wxConfigBase_SetExpandEnvVars(_obj: *mut ::libc::c_void,
                                         bDoIt: ::libc::c_int) -> ();
    pub fn wxConfigBase_SetPath(_obj: *mut ::libc::c_void,
                                strPath: *mut ::libc::c_void) -> ();
    pub fn wxConfigBase_SetRecordDefaults(_obj: *mut ::libc::c_void,
                                          bDoIt: ::libc::c_int) -> ();
    pub fn wxConfigBase_SetStyle(_obj: *mut ::libc::c_void,
                                 style: ::libc::c_int) -> ();
    pub fn wxConfigBase_SetVendorName(_obj: *mut ::libc::c_void,
                                      vendorName: *mut ::libc::c_void) -> ();
    pub fn wxConfigBase_WriteBool(_obj: *mut ::libc::c_void,
                                  key: *mut ::libc::c_void,
                                  value: ::libc::c_int) -> ::libc::c_int;
    pub fn wxConfigBase_WriteDouble(_obj: *mut ::libc::c_void,
                                    key: *mut ::libc::c_void,
                                    value: ::libc::c_double) -> ::libc::c_int;
    pub fn wxConfigBase_WriteInteger(_obj: *mut ::libc::c_void,
                                     key: *mut ::libc::c_void,
                                     value: ::libc::c_int) -> ::libc::c_int;
    pub fn wxConfigBase_WriteLong(_obj: *mut ::libc::c_void,
                                  key: *mut ::libc::c_void,
                                  value: ::libc::c_long) -> ::libc::c_int;
    pub fn wxConfigBase_WriteString(_obj: *mut ::libc::c_void,
                                    key: *mut ::libc::c_void,
                                    value: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxContextHelp_BeginContextHelp(_obj: *mut ::libc::c_void,
                                          win: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxContextHelp_Create(win: *mut ::libc::c_void,
                                beginHelp: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxContextHelp_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxContextHelp_EndContextHelp(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxContextHelpButton_Create(parent: *mut ::libc::c_void,
                                      id: ::libc::c_int, x: ::libc::c_int,
                                      y: ::libc::c_int, w: ::libc::c_int,
                                      h: ::libc::c_int, style: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxControl_Command(_obj: *mut ::libc::c_void,
                             event: *mut ::libc::c_void) -> ();
    pub fn wxControl_GetLabel(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxControl_SetLabel(_obj: *mut ::libc::c_void,
                              text: *mut ::libc::c_void) -> ();
    pub fn wxCriticalSection_Create() -> *mut ::libc::c_void;
    pub fn wxCriticalSection_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCriticalSection_Enter(_obj: *mut ::libc::c_void) -> ();
    pub fn wxCriticalSection_Leave(_obj: *mut ::libc::c_void) -> ();
    pub fn Cursor_CreateFromStock(_id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn Cursor_CreateFromImage(image: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn Cursor_CreateLoad(name: *mut ::libc::c_void, _type: ::libc::c_long,
                             width: ::libc::c_int, height: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDC_Blit(_obj: *mut ::libc::c_void, xdest: ::libc::c_int,
                     ydest: ::libc::c_int, width: ::libc::c_int,
                     height: ::libc::c_int, source: *mut ::libc::c_void,
                     xsrc: ::libc::c_int, ysrc: ::libc::c_int,
                     rop: ::libc::c_int, useMask: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDC_CalcBoundingBox(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int) -> ();
    pub fn wxDC_CanDrawBitmap(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_CanGetTextExtent(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_ComputeScaleAndOrigin(obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_CrossHair(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                          y: ::libc::c_int) -> ();
    pub fn wxDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_DestroyClippingRegion(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_DeviceToLogicalX(_obj: *mut ::libc::c_void, x: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDC_DeviceToLogicalXRel(_obj: *mut ::libc::c_void,
                                    x: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDC_DeviceToLogicalY(_obj: *mut ::libc::c_void, y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDC_DeviceToLogicalYRel(_obj: *mut ::libc::c_void,
                                    y: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDC_DrawArc(_obj: *mut ::libc::c_void, x1: ::libc::c_int,
                        y1: ::libc::c_int, x2: ::libc::c_int,
                        y2: ::libc::c_int, xc: ::libc::c_int,
                        yc: ::libc::c_int) -> ();
    pub fn wxDC_DrawBitmap(_obj: *mut ::libc::c_void,
                           bmp: *mut ::libc::c_void, x: ::libc::c_int,
                           y: ::libc::c_int, useMask: ::libc::c_int) -> ();
    pub fn wxDC_DrawCheckMark(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, width: ::libc::c_int,
                              height: ::libc::c_int) -> ();
    pub fn wxDC_DrawCircle(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                           y: ::libc::c_int, radius: ::libc::c_int) -> ();
    pub fn wxDC_DrawEllipse(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                            y: ::libc::c_int, width: ::libc::c_int,
                            height: ::libc::c_int) -> ();
    pub fn wxDC_DrawEllipticArc(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int, w: ::libc::c_int,
                                h: ::libc::c_int, sa: ::libc::c_double,
                                ea: ::libc::c_double) -> ();
    pub fn wxDC_DrawIcon(_obj: *mut ::libc::c_void, icon: *mut ::libc::c_void,
                         x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxDC_DrawLabel(_obj: *mut ::libc::c_void, str: *mut ::libc::c_void,
                          x: ::libc::c_int, y: ::libc::c_int,
                          w: ::libc::c_int, h: ::libc::c_int,
                          align: ::libc::c_int, indexAccel: ::libc::c_int)
     -> ();
    pub fn wxDC_DrawLabelBitmap(_obj: *mut ::libc::c_void,
                                str: *mut ::libc::c_void,
                                bmp: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int, w: ::libc::c_int,
                                h: ::libc::c_int, align: ::libc::c_int,
                                indexAccel: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDC_DrawLine(_obj: *mut ::libc::c_void, x1: ::libc::c_int,
                         y1: ::libc::c_int, x2: ::libc::c_int,
                         y2: ::libc::c_int) -> ();
    pub fn wxDC_DrawLines(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                          x: *mut ::libc::c_void, y: *mut ::libc::c_void,
                          xoffset: ::libc::c_int, yoffset: ::libc::c_int)
     -> ();
    pub fn wxDC_DrawPoint(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                          y: ::libc::c_int) -> ();
    pub fn wxDC_DrawPolygon(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                            x: *mut ::libc::c_void, y: *mut ::libc::c_void,
                            xoffset: ::libc::c_int, yoffset: ::libc::c_int,
                            fillStyle: ::libc::c_int) -> ();
    pub fn wxDC_DrawPolyPolygon(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                                count: *mut ::libc::c_void,
                                x: *mut ::libc::c_void,
                                y: *mut ::libc::c_void,
                                xoffset: ::libc::c_int,
                                yoffset: ::libc::c_int,
                                fillStyle: ::libc::c_int) -> ();
    pub fn wxDC_DrawRectangle(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, width: ::libc::c_int,
                              height: ::libc::c_int) -> ();
    pub fn wxDC_DrawRotatedText(_obj: *mut ::libc::c_void,
                                text: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int, angle: ::libc::c_double)
     -> ();
    pub fn wxDC_DrawRoundedRectangle(_obj: *mut ::libc::c_void,
                                     x: ::libc::c_int, y: ::libc::c_int,
                                     width: ::libc::c_int,
                                     height: ::libc::c_int,
                                     radius: ::libc::c_double) -> ();
    pub fn wxDC_DrawText(_obj: *mut ::libc::c_void, text: *mut ::libc::c_void,
                         x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxDC_EndDoc(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_EndPage(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_FloodFill(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                          y: ::libc::c_int, col: *mut ::libc::c_void,
                          style: ::libc::c_int) -> ();
    pub fn wxDC_GetBackground(_obj: *mut ::libc::c_void,
                              _ref: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetBackgroundMode(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_GetBrush(_obj: *mut ::libc::c_void, _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_GetCharHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_GetCharWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_GetClippingBox(_obj: *mut ::libc::c_void,
                               _x: *mut ::libc::c_void,
                               _y: *mut ::libc::c_void,
                               _w: *mut ::libc::c_void,
                               _h: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetDepth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_GetDeviceOrigin(_obj: *mut ::libc::c_void,
                                _x: *mut ::libc::c_void,
                                _y: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetFont(_obj: *mut ::libc::c_void, _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_GetLogicalFunction(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDC_GetLogicalOrigin(_obj: *mut ::libc::c_void,
                                 _x: *mut ::libc::c_void,
                                 _y: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetLogicalScale(_obj: *mut ::libc::c_void,
                                x: *mut ::libc::c_double,
                                y: *mut ::libc::c_double) -> ();
    pub fn wxDC_GetMapMode(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_GetPPI(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxDC_GetPen(_obj: *mut ::libc::c_void, _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_GetPixel(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                         y: ::libc::c_int, col: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDC_GetSize(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxDC_GetSizeMM(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxDC_GetTextBackground(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetTextExtent(_self: *mut ::libc::c_void,
                              string: *mut ::libc::c_void,
                              w: *mut ::libc::c_void, h: *mut ::libc::c_void,
                              descent: *mut ::libc::c_void,
                              externalLeading: *mut ::libc::c_void,
                              theFont: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetMultiLineTextExtent(_self: *mut ::libc::c_void,
                                       string: *mut ::libc::c_void,
                                       w: *mut ::libc::c_void,
                                       h: *mut ::libc::c_void,
                                       heightLine: *mut ::libc::c_void,
                                       theFont: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetTextForeground(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxDC_GetUserScale(_obj: *mut ::libc::c_void,
                             x: *mut ::libc::c_double,
                             y: *mut ::libc::c_double) -> ();
    pub fn wxDC_LogicalToDeviceX(_obj: *mut ::libc::c_void, x: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDC_LogicalToDeviceXRel(_obj: *mut ::libc::c_void,
                                    x: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDC_LogicalToDeviceY(_obj: *mut ::libc::c_void, y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDC_LogicalToDeviceYRel(_obj: *mut ::libc::c_void,
                                    y: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDC_MaxX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_MaxY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_MinX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_MinY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDC_ResetBoundingBox(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetAxisOrientation(_obj: *mut ::libc::c_void,
                                   xLeftRight: ::libc::c_int,
                                   yBottomUp: ::libc::c_int) -> ();
    pub fn wxDC_SetBackground(_obj: *mut ::libc::c_void,
                              brush: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetBackgroundMode(_obj: *mut ::libc::c_void,
                                  mode: ::libc::c_int) -> ();
    pub fn wxDC_SetBrush(_obj: *mut ::libc::c_void,
                         brush: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetClippingRegion(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                  y: ::libc::c_int, width: ::libc::c_int,
                                  height: ::libc::c_int) -> ();
    pub fn wxDC_SetClippingRegionFromRegion(_obj: *mut ::libc::c_void,
                                            region: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_SetDeviceClippingRegion(_obj: *mut ::libc::c_void,
                                        region: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetDeviceOrigin(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int) -> ();
    pub fn wxDC_SetFont(_obj: *mut ::libc::c_void, font: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_SetLogicalFunction(_obj: *mut ::libc::c_void,
                                   function: ::libc::c_int) -> ();
    pub fn wxDC_SetLogicalOrigin(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int) -> ();
    pub fn wxDC_SetLogicalScale(_obj: *mut ::libc::c_void,
                                x: ::libc::c_double, y: ::libc::c_double)
     -> ();
    pub fn wxDC_SetMapMode(_obj: *mut ::libc::c_void, mode: ::libc::c_int)
     -> ();
    pub fn wxDC_SetPalette(_obj: *mut ::libc::c_void,
                           palette: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetPen(_obj: *mut ::libc::c_void, pen: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_SetTextBackground(_obj: *mut ::libc::c_void,
                                  colour: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetTextForeground(_obj: *mut ::libc::c_void,
                                  colour: *mut ::libc::c_void) -> ();
    pub fn wxDC_SetUserScale(_obj: *mut ::libc::c_void, x: ::libc::c_double,
                             y: ::libc::c_double) -> ();
    pub fn wxDC_StartDoc(_obj: *mut ::libc::c_void, msg: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDC_StartPage(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDataFormat_CreateFromId(name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDataFormat_CreateFromType(typ: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDataFormat_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDataFormat_GetId(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDataFormat_GetType(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDataFormat_IsEqual(_obj: *mut ::libc::c_void,
                                other: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDataFormat_SetId(_obj: *mut ::libc::c_void,
                              id: *mut ::libc::c_void) -> ();
    pub fn wxDataFormat_SetType(_obj: *mut ::libc::c_void, typ: ::libc::c_int)
     -> ();
    pub fn wxDataObjectComposite_Add(_obj: *mut ::libc::c_void,
                                     _dat: *mut ::libc::c_void,
                                     _preferred: ::libc::c_int) -> ();
    pub fn wxDataObjectComposite_Create() -> *mut ::libc::c_void;
    pub fn wxDataObjectComposite_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_AddDate(_obj: *mut ::libc::c_void,
                              diff: *mut ::libc::c_void,
                              _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_AddDateValues(_obj: *mut ::libc::c_void,
                                    _yrs: ::libc::c_int, _mnt: ::libc::c_int,
                                    _wek: ::libc::c_int, _day: ::libc::c_int)
     -> ();
    pub fn wxDateTime_AddTime(_obj: *mut ::libc::c_void,
                              diff: *mut ::libc::c_void,
                              _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_AddTimeValues(_obj: *mut ::libc::c_void,
                                    _hrs: ::libc::c_int, _min: ::libc::c_int,
                                    _sec: ::libc::c_int, _mls: ::libc::c_int)
     -> ();
    pub fn wxDateTime_ConvertYearToBC(year: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_Create() -> *mut ::libc::c_void;
    pub fn wxDateTime_Format(_obj: *mut ::libc::c_void,
                             format: *mut ::libc::c_void, tz: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_FormatDate(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_FormatISODate(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_FormatISOTime(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_FormatTime(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_GetAmString() -> *mut ::libc::c_void;
    pub fn wxDateTime_GetBeginDST(year: ::libc::c_int, country: ::libc::c_int,
                                  dt: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetCentury(year: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_GetCountry() -> ::libc::c_int;
    pub fn wxDateTime_GetCurrentMonth(cal: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_GetCurrentYear(cal: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_GetDay(_obj: *mut ::libc::c_void, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetDayOfYear(_obj: *mut ::libc::c_void,
                                   tz: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_GetEndDST(year: ::libc::c_int, country: ::libc::c_int,
                                dt: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetHour(_obj: *mut ::libc::c_void, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetLastMonthDay(_obj: *mut ::libc::c_void,
                                      month: ::libc::c_int,
                                      year: ::libc::c_int,
                                      _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetLastWeekDay(_obj: *mut ::libc::c_void,
                                     weekday: ::libc::c_int,
                                     month: ::libc::c_int,
                                     year: ::libc::c_int,
                                     _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetMillisecond(_obj: *mut ::libc::c_void,
                                     tz: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_GetMinute(_obj: *mut ::libc::c_void, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetMonth(_obj: *mut ::libc::c_void, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetMonthName(month: ::libc::c_int, flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_GetNextWeekDay(_obj: *mut ::libc::c_void,
                                     weekday: ::libc::c_int,
                                     _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetNumberOfDays(year: ::libc::c_int, cal: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetNumberOfDaysMonth(month: ::libc::c_int,
                                           year: ::libc::c_int,
                                           cal: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetPmString() -> *mut ::libc::c_void;
    pub fn wxDateTime_GetPrevWeekDay(_obj: *mut ::libc::c_void,
                                     weekday: ::libc::c_int,
                                     _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetSecond(_obj: *mut ::libc::c_void, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetTicks(_obj: *mut ::libc::c_void) -> time_t;
    pub fn wxDateTime_GetTimeNow() -> ::libc::c_int;
    pub fn wxDateTime_GetValue(_obj: *mut ::libc::c_void,
                               hi_long: *mut ::libc::c_void,
                               lo_long: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetWeekDay(_obj: *mut ::libc::c_void,
                                 weekday: ::libc::c_int, n: ::libc::c_int,
                                 month: ::libc::c_int, year: ::libc::c_int,
                                 _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetWeekDayInSameWeek(_obj: *mut ::libc::c_void,
                                           weekday: ::libc::c_int,
                                           _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_GetWeekDayName(weekday: ::libc::c_int,
                                     flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_GetWeekDayTZ(_obj: *mut ::libc::c_void,
                                   tz: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_GetWeekOfMonth(_obj: *mut ::libc::c_void,
                                     flags: ::libc::c_int, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetWeekOfYear(_obj: *mut ::libc::c_void,
                                    flags: ::libc::c_int, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_GetYear(_obj: *mut ::libc::c_void, tz: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_IsBetween(_obj: *mut ::libc::c_void,
                                t1: *mut ::libc::c_void,
                                t2: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDateTime_IsDST(_obj: *mut ::libc::c_void, country: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_IsDSTApplicable(year: ::libc::c_int,
                                      country: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_IsEarlierThan(_obj: *mut ::libc::c_void,
                                    datetime: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDateTime_IsEqualTo(_obj: *mut ::libc::c_void,
                                datetime: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDateTime_IsEqualUpTo(_obj: *mut ::libc::c_void,
                                  dt: *mut ::libc::c_void,
                                  ts: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDateTime_IsGregorianDate(_obj: *mut ::libc::c_void,
                                      country: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_IsLaterThan(_obj: *mut ::libc::c_void,
                                  datetime: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDateTime_IsLeapYear(year: ::libc::c_int, cal: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_IsSameDate(_obj: *mut ::libc::c_void,
                                 dt: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDateTime_IsSameTime(_obj: *mut ::libc::c_void,
                                 dt: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDateTime_IsStrictlyBetween(_obj: *mut ::libc::c_void,
                                        t1: *mut ::libc::c_void,
                                        t2: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDateTime_IsValid(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDateTime_IsWestEuropeanCountry(country: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_IsWorkDay(_obj: *mut ::libc::c_void,
                                country: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_MakeGMT(_obj: *mut ::libc::c_void, noDST: ::libc::c_int)
     -> ();
    pub fn wxDateTime_MakeTimezone(_obj: *mut ::libc::c_void,
                                   tz: ::libc::c_int, noDST: ::libc::c_int)
     -> ();
    pub fn wxDateTime_Now(dt: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_ParseDate(_obj: *mut ::libc::c_void,
                                date: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_ParseDateTime(_obj: *mut ::libc::c_void,
                                    datetime: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_ParseFormat(_obj: *mut ::libc::c_void,
                                  date: *mut ::libc::c_void,
                                  format: *mut ::libc::c_void,
                                  dateDef: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_ParseRfc822Date(_obj: *mut ::libc::c_void,
                                      date: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_ParseTime(_obj: *mut ::libc::c_void,
                                time: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDateTime_ResetTime(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_Set(_obj: *mut ::libc::c_void, day: ::libc::c_int,
                          month: ::libc::c_int, year: ::libc::c_int,
                          hour: ::libc::c_int, minute: ::libc::c_int,
                          second: ::libc::c_int, millisec: ::libc::c_int)
     -> ();
    pub fn wxDateTime_SetCountry(country: ::libc::c_int) -> ();
    pub fn wxDateTime_SetDay(_obj: *mut ::libc::c_void, day: ::libc::c_int)
     -> ();
    pub fn wxDateTime_SetHour(_obj: *mut ::libc::c_void, hour: ::libc::c_int)
     -> ();
    pub fn wxDateTime_SetMillisecond(_obj: *mut ::libc::c_void,
                                     millisecond: ::libc::c_int) -> ();
    pub fn wxDateTime_SetMinute(_obj: *mut ::libc::c_void,
                                minute: ::libc::c_int) -> ();
    pub fn wxDateTime_SetMonth(_obj: *mut ::libc::c_void,
                               month: ::libc::c_int) -> ();
    pub fn wxDateTime_SetSecond(_obj: *mut ::libc::c_void,
                                second: ::libc::c_int) -> ();
    pub fn wxDateTime_SetTime(_obj: *mut ::libc::c_void, hour: ::libc::c_int,
                              minute: ::libc::c_int, second: ::libc::c_int,
                              millisec: ::libc::c_int) -> ();
    pub fn wxDateTime_SetToCurrent(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_SetToLastMonthDay(_obj: *mut ::libc::c_void,
                                        month: ::libc::c_int,
                                        year: ::libc::c_int) -> ();
    pub fn wxDateTime_SetToLastWeekDay(_obj: *mut ::libc::c_void,
                                       weekday: ::libc::c_int,
                                       month: ::libc::c_int,
                                       year: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDateTime_SetToNextWeekDay(_obj: *mut ::libc::c_void,
                                       weekday: ::libc::c_int) -> ();
    pub fn wxDateTime_SetToPrevWeekDay(_obj: *mut ::libc::c_void,
                                       weekday: ::libc::c_int) -> ();
    pub fn wxDateTime_SetToWeekDay(_obj: *mut ::libc::c_void,
                                   weekday: ::libc::c_int, n: ::libc::c_int,
                                   month: ::libc::c_int, year: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDateTime_SetToWeekDayInSameWeek(_obj: *mut ::libc::c_void,
                                             weekday: ::libc::c_int) -> ();
    pub fn wxDateTime_SetYear(_obj: *mut ::libc::c_void, year: ::libc::c_int)
     -> ();
    pub fn wxDateTime_SubtractDate(_obj: *mut ::libc::c_void,
                                   diff: *mut ::libc::c_void,
                                   _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_SubtractTime(_obj: *mut ::libc::c_void,
                                   diff: *mut ::libc::c_void,
                                   _ref: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_ToGMT(_obj: *mut ::libc::c_void, noDST: ::libc::c_int)
     -> ();
    pub fn wxDateTime_ToTimezone(_obj: *mut ::libc::c_void, tz: ::libc::c_int,
                                 noDST: ::libc::c_int) -> ();
    pub fn wxDateTime_Today(dt: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_UNow(dt: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_wxDateTime(hi_long: ::libc::c_int,
                                 lo_long: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDialUpEvent_IsConnectedEvent(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpEvent_IsOwnEvent(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpManager_CancelDialing(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpManager_Create() -> *mut ::libc::c_void;
    pub fn wxDialUpManager_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDialUpManager_Dial(_obj: *mut ::libc::c_void,
                                nameOfISP: *mut ::libc::c_void,
                                username: *mut ::libc::c_void,
                                password: *mut ::libc::c_void,
                                async: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDialUpManager_DisableAutoCheckOnlineStatus(_obj:
                                                            *mut ::libc::c_void)
     -> ();
    pub fn wxDialUpManager_EnableAutoCheckOnlineStatus(_obj:
                                                           *mut ::libc::c_void,
                                                       nSeconds:
                                                           ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDialUpManager_GetISPNames(_obj: *mut ::libc::c_void,
                                       _lst: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpManager_HangUp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDialUpManager_IsAlwaysOnline(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpManager_IsDialing(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpManager_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDialUpManager_IsOnline(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDialUpManager_SetConnectCommand(_obj: *mut ::libc::c_void,
                                             commandDial: *mut ::libc::c_void,
                                             commandHangup:
                                                 *mut ::libc::c_void) -> ();
    pub fn wxDialUpManager_SetOnlineStatus(_obj: *mut ::libc::c_void,
                                           isOnline: ::libc::c_int) -> ();
    pub fn wxDialUpManager_SetWellKnownHost(_obj: *mut ::libc::c_void,
                                            hostname: *mut ::libc::c_void,
                                            portno: ::libc::c_int) -> ();
    pub fn wxDialog_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                           _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                           _top: ::libc::c_int, _wdt: ::libc::c_int,
                           _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDialog_EndModal(_obj: *mut ::libc::c_void,
                             retCode: ::libc::c_int) -> ();
    pub fn wxDialog_GetReturnCode(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDialog_IsModal(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDialog_SetReturnCode(_obj: *mut ::libc::c_void,
                                  returnCode: ::libc::c_int) -> ();
    pub fn wxDialog_ShowModal(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDirDialog_Create(_prt: *mut ::libc::c_void,
                              _msg: *mut ::libc::c_void,
                              _dir: *mut ::libc::c_void, _lft: ::libc::c_int,
                              _top: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDirDialog_GetMessage(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDirDialog_GetPath(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDirDialog_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDirDialog_SetMessage(_obj: *mut ::libc::c_void,
                                  msg: *mut ::libc::c_void) -> ();
    pub fn wxDirDialog_SetPath(_obj: *mut ::libc::c_void,
                               pth: *mut ::libc::c_void) -> ();
    pub fn wxDirDialog_SetStyle(_obj: *mut ::libc::c_void,
                                style: ::libc::c_int) -> ();
    pub fn wxDrawControl_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                                _lft: ::libc::c_int, _top: ::libc::c_int,
                                _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxDrawWindow_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                               _lft: ::libc::c_int, _top: ::libc::c_int,
                               _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                               _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn DropSource_Create(data: *mut ::libc::c_void,
                             win: *mut ::libc::c_void,
                             copy: *mut ::libc::c_void,
                             _move: *mut ::libc::c_void,
                             none: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn DropSource_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn DropSource_DoDragDrop(_obj: *mut ::libc::c_void,
                                 _move: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDropTarget_GetData(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDropTarget_SetDataObject(_obj: *mut ::libc::c_void,
                                      _dat: *mut ::libc::c_void) -> ();
    pub fn wxDynToolInfo_Index(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDynToolInfo_RealSize(_obj: *mut ::libc::c_void,
                                  _w: *mut ::libc::c_void,
                                  _h: *mut ::libc::c_void) -> ();
    pub fn wxDynToolInfo_pToolWnd(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicSashWindow_Create(parent: *mut ::libc::c_void,
                                      id: ::libc::c_int, x: ::libc::c_int,
                                      y: ::libc::c_int, w: ::libc::c_int,
                                      h: ::libc::c_int, style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDynamicSashWindow_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDynamicSashWindow_GetHScrollBar(_obj: *mut ::libc::c_void,
                                             child: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicSashWindow_GetVScrollBar(_obj: *mut ::libc::c_void,
                                             child: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_AddSeparator(_obj: *mut ::libc::c_void,
                                         pSepartorWnd: *mut ::libc::c_void)
     -> ();
    pub fn wxDynamicToolBar_AddTool(_obj: *mut ::libc::c_void,
                                    toolIndex: ::libc::c_int,
                                    pToolWindow: *mut ::libc::c_void,
                                    w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_AddToolBitmap(_obj: *mut ::libc::c_void,
                                          toolIndex: ::libc::c_int,
                                          bitmap: *mut ::libc::c_void,
                                          pushedBitmap: *mut ::libc::c_void,
                                          toggle: ::libc::c_int,
                                          x: ::libc::c_int, y: ::libc::c_int,
                                          clientData: *mut ::libc::c_void,
                                          helpString1: *mut ::libc::c_void,
                                          helpString2: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_AddToolImage(_obj: *mut ::libc::c_void,
                                         toolIndex: ::libc::c_int,
                                         imageFileName: *mut ::libc::c_void,
                                         imageFileType: ::libc::c_int,
                                         labelText: *mut ::libc::c_void,
                                         alignTextRight: ::libc::c_int,
                                         isFlat: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_AddToolLabel(_obj: *mut ::libc::c_void,
                                         toolIndex: ::libc::c_int,
                                         labelBmp: *mut ::libc::c_void,
                                         labelText: *mut ::libc::c_void,
                                         alignTextRight: ::libc::c_int,
                                         isFlat: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_Create(parent: *mut ::libc::c_void,
                                   id: ::libc::c_int, x: ::libc::c_int,
                                   y: ::libc::c_int, w: ::libc::c_int,
                                   h: ::libc::c_int, style: ::libc::c_int,
                                   orientation: ::libc::c_int,
                                   RowsOrColumns: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_CreateDefaultLayout(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_CreateParams(_obj: *mut ::libc::c_void,
                                         parent: *mut ::libc::c_void,
                                         id: ::libc::c_int, x: ::libc::c_int,
                                         y: ::libc::c_int, w: ::libc::c_int,
                                         h: ::libc::c_int,
                                         style: ::libc::c_int,
                                         orientation: ::libc::c_int,
                                         RowsOrColumns: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDynamicToolBar_CreateTool(_obj: *mut ::libc::c_void,
                                       id: ::libc::c_int,
                                       label: *mut ::libc::c_void,
                                       bmpNormal: *mut ::libc::c_void,
                                       bmpDisabled: *mut ::libc::c_void,
                                       kind: ::libc::c_int,
                                       clientData: *mut ::libc::c_void,
                                       shortHelp: *mut ::libc::c_void,
                                       longHelp: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_CreateToolControl(_obj: *mut ::libc::c_void,
                                              control: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDynamicToolBar_DoDeleteTool(_obj: *mut ::libc::c_void,
                                         pos: ::libc::c_int,
                                         tool: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDynamicToolBar_DoEnableTool(_obj: *mut ::libc::c_void,
                                         tool: *mut ::libc::c_void,
                                         enable: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_DoInsertTool(_obj: *mut ::libc::c_void,
                                         pos: ::libc::c_int,
                                         tool: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDynamicToolBar_DoSetToggle(_obj: *mut ::libc::c_void,
                                        tool: *mut ::libc::c_void,
                                        toggle: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_DoToggleTool(_obj: *mut ::libc::c_void,
                                         tool: *mut ::libc::c_void,
                                         toggle: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_DrawSeparator(_obj: *mut ::libc::c_void,
                                          info: *mut ::libc::c_void,
                                          dc: *mut ::libc::c_void) -> ();
    pub fn wxDynamicToolBar_EnableTool(_obj: *mut ::libc::c_void,
                                       toolIndex: ::libc::c_int,
                                       enable: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_FindToolForPosition(_obj: *mut ::libc::c_void,
                                                x: ::libc::c_int,
                                                y: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_GetPreferredDim(_obj: *mut ::libc::c_void,
                                            gw: ::libc::c_int,
                                            gh: ::libc::c_int,
                                            pw: *mut ::libc::c_void,
                                            ph: *mut ::libc::c_void) -> ();
    pub fn wxDynamicToolBar_GetToolInfo(_obj: *mut ::libc::c_void,
                                        toolIndex: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDynamicToolBar_Layout(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDynamicToolBar_RemoveTool(_obj: *mut ::libc::c_void,
                                       toolIndex: ::libc::c_int) -> ();
    pub fn wxDynamicToolBar_SetLayout(_obj: *mut ::libc::c_void,
                                      pLayout: *mut ::libc::c_void) -> ();
    pub fn wxEditableListBox_Create(parent: *mut ::libc::c_void,
                                    id: ::libc::c_int,
                                    label: *mut ::libc::c_void,
                                    x: ::libc::c_int, y: ::libc::c_int,
                                    w: ::libc::c_int, h: ::libc::c_int,
                                    style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_GetDelButton(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_GetDownButton(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_GetEditButton(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_GetListCtrl(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_GetNewButton(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_GetStrings(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxEditableListBox_GetUpButton(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEditableListBox_SetStrings(_obj: *mut ::libc::c_void,
                                        strings: *mut ::libc::c_void,
                                        _n: ::libc::c_int) -> ();
    pub fn wxEncodingConverter_Convert(_obj: *mut ::libc::c_void,
                                       input: *mut ::libc::c_void,
                                       output: *mut ::libc::c_void) -> ();
    pub fn wxEncodingConverter_Create() -> *mut ::libc::c_void;
    pub fn wxEncodingConverter_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxEncodingConverter_GetAllEquivalents(_obj: *mut ::libc::c_void,
                                                 enc: ::libc::c_int,
                                                 _lst: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxEncodingConverter_GetPlatformEquivalents(_obj:
                                                          *mut ::libc::c_void,
                                                      enc: ::libc::c_int,
                                                      platform: ::libc::c_int,
                                                      _lst:
                                                          *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxEncodingConverter_Init(_obj: *mut ::libc::c_void,
                                    input_enc: ::libc::c_int,
                                    output_enc: ::libc::c_int,
                                    method: ::libc::c_int) -> ::libc::c_int;
    pub fn wxEraseEvent_CopyObject(_obj: *mut ::libc::c_void,
                                   obj: *mut ::libc::c_void) -> ();
    pub fn wxEraseEvent_GetDC(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEvent_CopyObject(_obj: *mut ::libc::c_void,
                              object_dest: *mut ::libc::c_void) -> ();
    pub fn wxEvent_GetEventObject(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEvent_GetEventType(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxEvent_GetId(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxEvent_GetSkipped(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxEvent_GetTimestamp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxEvent_IsCommandEvent(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxEvent_NewEventType() -> ::libc::c_int;
    pub fn wxEvent_SetEventObject(_obj: *mut ::libc::c_void,
                                  obj: *mut ::libc::c_void) -> ();
    pub fn wxEvent_SetEventType(_obj: *mut ::libc::c_void, typ: ::libc::c_int)
     -> ();
    pub fn wxEvent_SetId(_obj: *mut ::libc::c_void, Id: ::libc::c_int) -> ();
    pub fn wxEvent_SetTimestamp(_obj: *mut ::libc::c_void, ts: ::libc::c_int)
     -> ();
    pub fn wxEvent_Skip(_obj: *mut ::libc::c_void) -> ();
    pub fn wxEvtHandler_AddPendingEvent(_obj: *mut ::libc::c_void,
                                        event: *mut ::libc::c_void) -> ();
    pub fn wxEvtHandler_Connect(_obj: *mut ::libc::c_void,
                                first: ::libc::c_int, last: ::libc::c_int,
                                _type: ::libc::c_int,
                                data: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxEvtHandler_Create() -> *mut ::libc::c_void;
    pub fn wxEvtHandler_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxEvtHandler_Disconnect(_obj: *mut ::libc::c_void,
                                   first: ::libc::c_int, last: ::libc::c_int,
                                   _type: ::libc::c_int, id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxEvtHandler_GetEvtHandlerEnabled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxEvtHandler_GetNextHandler(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEvtHandler_GetPreviousHandler(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEvtHandler_ProcessEvent(_obj: *mut ::libc::c_void,
                                     event: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxEvtHandler_ProcessPendingEvents(_obj: *mut ::libc::c_void) -> ();
    pub fn wxEvtHandler_SetEvtHandlerEnabled(_obj: *mut ::libc::c_void,
                                             enabled: ::libc::c_int) -> ();
    pub fn wxEvtHandler_SetNextHandler(_obj: *mut ::libc::c_void,
                                       handler: *mut ::libc::c_void) -> ();
    pub fn wxEvtHandler_SetPreviousHandler(_obj: *mut ::libc::c_void,
                                           handler: *mut ::libc::c_void)
     -> ();
    pub fn FileDataObject_AddFile(_obj: *mut ::libc::c_void,
                                  _fle: *mut ::libc::c_void) -> ();
    pub fn FileDataObject_Create(_cnt: ::libc::c_int,
                                 _lst: *mut *mut ::libc::c_char)
     -> *mut ::libc::c_void;
    pub fn FileDataObject_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn FileDataObject_GetFilenames(_obj: *mut ::libc::c_void,
                                       _lst: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileDialog_Create(_prt: *mut ::libc::c_void,
                               _msg: *mut ::libc::c_void,
                               _dir: *mut ::libc::c_void,
                               _fle: *mut ::libc::c_void,
                               _wcd: *mut ::libc::c_void, _lft: ::libc::c_int,
                               _top: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFileDialog_GetDirectory(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileDialog_GetFilename(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileDialog_GetFilenames(_obj: *mut ::libc::c_void,
                                     paths: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileDialog_GetFilterIndex(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileDialog_GetMessage(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileDialog_GetPath(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileDialog_GetPaths(_obj: *mut ::libc::c_void,
                                 paths: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFileDialog_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFileDialog_GetWildcard(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileDialog_SetDirectory(_obj: *mut ::libc::c_void,
                                     dir: *mut ::libc::c_void) -> ();
    pub fn wxFileDialog_SetFilename(_obj: *mut ::libc::c_void,
                                    name: *mut ::libc::c_void) -> ();
    pub fn wxFileDialog_SetFilterIndex(_obj: *mut ::libc::c_void,
                                       filterIndex: ::libc::c_int) -> ();
    pub fn wxFileDialog_SetMessage(_obj: *mut ::libc::c_void,
                                   message: *mut ::libc::c_void) -> ();
    pub fn wxFileDialog_SetPath(_obj: *mut ::libc::c_void,
                                path: *mut ::libc::c_void) -> ();
    pub fn wxFileDialog_SetStyle(_obj: *mut ::libc::c_void,
                                 style: ::libc::c_int) -> ();
    pub fn wxFileDialog_SetWildcard(_obj: *mut ::libc::c_void,
                                    wildCard: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_AddFileToHistory(_obj: *mut ::libc::c_void,
                                          file: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_AddFilesToMenu(_obj: *mut ::libc::c_void,
                                        menu: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_Create(maxFiles: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFileHistory_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_GetCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFileHistory_GetHistoryFile(_obj: *mut ::libc::c_void,
                                        i: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFileHistory_GetMaxFiles(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileHistory_GetMenus(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFileHistory_Load(_obj: *mut ::libc::c_void,
                              config: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_RemoveFileFromHistory(_obj: *mut ::libc::c_void,
                                               i: ::libc::c_int) -> ();
    pub fn wxFileHistory_RemoveMenu(_obj: *mut ::libc::c_void,
                                    menu: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_Save(_obj: *mut ::libc::c_void,
                              config: *mut ::libc::c_void) -> ();
    pub fn wxFileHistory_UseMenu(_obj: *mut ::libc::c_void,
                                 menu: *mut ::libc::c_void) -> ();
    pub fn wxFileType_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFileType_ExpandCommand(_obj: *mut ::libc::c_void,
                                    _cmd: *mut ::libc::c_void,
                                    _params: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileType_GetDescription(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileType_GetExtensions(_obj: *mut ::libc::c_void,
                                    _lst: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileType_GetIcon(_obj: *mut ::libc::c_void,
                              icon: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFileType_GetMimeType(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileType_GetMimeTypes(_obj: *mut ::libc::c_void,
                                   _lst: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileType_GetOpenCommand(_obj: *mut ::libc::c_void,
                                     _buf: *mut ::libc::c_void,
                                     _params: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFileType_GetPrintCommand(_obj: *mut ::libc::c_void,
                                      _buf: *mut ::libc::c_void,
                                      _params: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFindDialogEvent_GetFindString(_obj: *mut ::libc::c_void,
                                           _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFindDialogEvent_GetFlags(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFindDialogEvent_GetReplaceString(_obj: *mut ::libc::c_void,
                                              _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFindReplaceData_Create(flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFindReplaceData_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxFindReplaceData_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFindReplaceData_GetFindString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFindReplaceData_GetFlags(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFindReplaceData_GetReplaceString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFindReplaceData_SetFindString(_obj: *mut ::libc::c_void,
                                           str: *mut ::libc::c_void) -> ();
    pub fn wxFindReplaceData_SetFlags(_obj: *mut ::libc::c_void,
                                      flags: ::libc::c_int) -> ();
    pub fn wxFindReplaceData_SetReplaceString(_obj: *mut ::libc::c_void,
                                              str: *mut ::libc::c_void) -> ();
    pub fn wxFindReplaceDialog_Create(parent: *mut ::libc::c_void,
                                      data: *mut ::libc::c_void,
                                      title: *mut ::libc::c_void,
                                      style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFindReplaceDialog_GetData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFindReplaceDialog_SetData(_obj: *mut ::libc::c_void,
                                       data: *mut ::libc::c_void) -> ();
    pub fn wxFlexGridSizer_AddGrowableCol(_obj: *mut ::libc::c_void,
                                          idx: size_t) -> ();
    pub fn wxFlexGridSizer_AddGrowableRow(_obj: *mut ::libc::c_void,
                                          idx: size_t) -> ();
    pub fn wxFlexGridSizer_CalcMin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFlexGridSizer_Create(rows: ::libc::c_int, cols: ::libc::c_int,
                                  vgap: ::libc::c_int, hgap: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFlexGridSizer_RecalcSizes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFlexGridSizer_RemoveGrowableCol(_obj: *mut ::libc::c_void,
                                             idx: size_t) -> ();
    pub fn wxFlexGridSizer_RemoveGrowableRow(_obj: *mut ::libc::c_void,
                                             idx: size_t) -> ();
    pub fn wxFont_Create(pointSize: ::libc::c_int, family: ::libc::c_int,
                         style: ::libc::c_int, weight: ::libc::c_int,
                         underlined: ::libc::c_int, face: *mut ::libc::c_void,
                         enc: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxFont_CreateFromStock(id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxFont_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxFont_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFont_GetDefaultEncoding(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFont_GetEncoding(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_GetFaceName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFont_GetFamily(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_GetFamilyString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFont_GetPointSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_GetStyleString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFont_GetUnderlined(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_GetWeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_GetWeightString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFont_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_SetDefaultEncoding(_obj: *mut ::libc::c_void,
                                     encoding: ::libc::c_int) -> ();
    pub fn wxFont_SetEncoding(_obj: *mut ::libc::c_void,
                              encoding: ::libc::c_int) -> ();
    pub fn wxFont_SetFaceName(_obj: *mut ::libc::c_void,
                              faceName: *mut ::libc::c_void) -> ();
    pub fn wxFont_SetFamily(_obj: *mut ::libc::c_void, family: ::libc::c_int)
     -> ();
    pub fn wxFont_SetPointSize(_obj: *mut ::libc::c_void,
                               pointSize: ::libc::c_int) -> ();
    pub fn wxFont_SetStyle(_obj: *mut ::libc::c_void, style: ::libc::c_int)
     -> ();
    pub fn wxFont_SetUnderlined(_obj: *mut ::libc::c_void,
                                underlined: ::libc::c_int) -> ();
    pub fn wxFont_SetWeight(_obj: *mut ::libc::c_void, weight: ::libc::c_int)
     -> ();
    pub fn wxFontData_Create() -> *mut ::libc::c_void;
    pub fn wxFontData_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFontData_EnableEffects(_obj: *mut ::libc::c_void,
                                    flag: ::libc::c_int) -> ();
    pub fn wxFontData_GetAllowSymbols(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFontData_GetChosenFont(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxFontData_GetColour(_obj: *mut ::libc::c_void,
                                _ref: *mut ::libc::c_void) -> ();
    pub fn wxFontData_GetEnableEffects(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFontData_GetEncoding(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFontData_GetInitialFont(_obj: *mut ::libc::c_void,
                                     _ref: *mut ::libc::c_void) -> ();
    pub fn wxFontData_GetShowHelp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFontData_SetAllowSymbols(_obj: *mut ::libc::c_void,
                                      flag: ::libc::c_int) -> ();
    pub fn wxFontData_SetChosenFont(_obj: *mut ::libc::c_void,
                                    font: *mut ::libc::c_void) -> ();
    pub fn wxFontData_SetColour(_obj: *mut ::libc::c_void,
                                colour: *mut ::libc::c_void) -> ();
    pub fn wxFontData_SetEncoding(_obj: *mut ::libc::c_void,
                                  encoding: ::libc::c_int) -> ();
    pub fn wxFontData_SetInitialFont(_obj: *mut ::libc::c_void,
                                     font: *mut ::libc::c_void) -> ();
    pub fn wxFontData_SetRange(_obj: *mut ::libc::c_void,
                               minRange: ::libc::c_int,
                               maxRange: ::libc::c_int) -> ();
    pub fn wxFontData_SetShowHelp(_obj: *mut ::libc::c_void,
                                  flag: ::libc::c_int) -> ();
    pub fn wxFontDialog_Create(_prt: *mut ::libc::c_void,
                               fnt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFontDialog_GetFontData(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxFontEnumerator_Create(_obj: *mut ::libc::c_void,
                                   _fnc: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFontEnumerator_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFontEnumerator_EnumerateEncodings(_obj: *mut ::libc::c_void,
                                               facename: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFontEnumerator_EnumerateFacenames(_obj: *mut ::libc::c_void,
                                               encoding: ::libc::c_int,
                                               fixedWidthOnly: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxFontMapper_Create() -> *mut ::libc::c_void;
    pub fn wxFontMapper_GetAltForEncoding(_obj: *mut ::libc::c_void,
                                          encoding: ::libc::c_int,
                                          alt_encoding: *mut ::libc::c_void,
                                          _buf: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFontMapper_IsEncodingAvailable(_obj: *mut ::libc::c_void,
                                            encoding: ::libc::c_int,
                                            _buf: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFrame_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                          _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                          _top: ::libc::c_int, _wdt: ::libc::c_int,
                          _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFrame_CreateStatusBar(_obj: *mut ::libc::c_void,
                                   number: ::libc::c_int,
                                   style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFrame_CreateToolBar(_obj: *mut ::libc::c_void,
                                 style: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxFrame_GetClientAreaOrigin_left(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFrame_GetClientAreaOrigin_top(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFrame_GetMenuBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrame_GetStatusBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrame_GetToolBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrame_Restore(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrame_SetMenuBar(_obj: *mut ::libc::c_void,
                              menubar: *mut ::libc::c_void) -> ();
    pub fn wxFrame_SetStatusBar(_obj: *mut ::libc::c_void,
                                statBar: *mut ::libc::c_void) -> ();
    pub fn wxFrame_SetStatusText(_obj: *mut ::libc::c_void,
                                 _txt: *mut ::libc::c_void,
                                 _number: ::libc::c_int) -> ();
    pub fn wxFrame_SetStatusWidths(_obj: *mut ::libc::c_void,
                                   _n: ::libc::c_int,
                                   _widths_field: *mut ::libc::c_void) -> ();
    pub fn wxFrame_SetToolBar(_obj: *mut ::libc::c_void,
                              _toolbar: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_Activate(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_AddBar(_obj: *mut ::libc::c_void,
                                pBarWnd: *mut ::libc::c_void,
                                dimInfo: *mut ::libc::c_void,
                                alignment: ::libc::c_int,
                                rowNo: ::libc::c_int,
                                columnPos: ::libc::c_int,
                                name: *mut ::libc::c_void,
                                spyEvents: ::libc::c_int,
                                state: ::libc::c_int) -> ();
    pub fn wxFrameLayout_AddPlugin(_obj: *mut ::libc::c_void,
                                   pPlInfo: *mut ::libc::c_void,
                                   paneMask: ::libc::c_int) -> ();
    pub fn wxFrameLayout_AddPluginBefore(_obj: *mut ::libc::c_void,
                                         pNextPlInfo: *mut ::libc::c_void,
                                         pPlInfo: *mut ::libc::c_void,
                                         paneMask: ::libc::c_int) -> ();
    pub fn wxFrameLayout_ApplyBarProperties(_obj: *mut ::libc::c_void,
                                            pBar: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_CaptureEventsForPane(_obj: *mut ::libc::c_void,
                                              toPane: *mut ::libc::c_void)
     -> ();
    pub fn wxFrameLayout_CaptureEventsForPlugin(_obj: *mut ::libc::c_void,
                                                pPlugin: *mut ::libc::c_void)
     -> ();
    pub fn wxFrameLayout_Create(pParentFrame: *mut ::libc::c_void,
                                pFrameClient: *mut ::libc::c_void,
                                activateNow: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_Deactivate(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_DestroyBarWindows(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_EnableFloating(_obj: *mut ::libc::c_void,
                                        enable: ::libc::c_int) -> ();
    pub fn wxFrameLayout_FindBarByName(_obj: *mut ::libc::c_void,
                                       name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_FindBarByWindow(_obj: *mut ::libc::c_void,
                                         pWnd: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_FindPlugin(_obj: *mut ::libc::c_void,
                                    pPlInfo: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_FirePluginEvent(_obj: *mut ::libc::c_void,
                                         event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_GetBars(_obj: *mut ::libc::c_void,
                                 _ref: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFrameLayout_GetClientHeight(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFrameLayout_GetClientRect(_obj: *mut ::libc::c_void,
                                       _x: *mut ::libc::c_void,
                                       _y: *mut ::libc::c_void,
                                       _w: *mut ::libc::c_void,
                                       _h: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_GetClientWidth(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFrameLayout_GetFrameClient(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_GetPane(_obj: *mut ::libc::c_void,
                                 alignment: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_GetPaneProperties(_obj: *mut ::libc::c_void,
                                           props: *mut ::libc::c_void,
                                           alignment: ::libc::c_int) -> ();
    pub fn wxFrameLayout_GetParentFrame(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_GetTopPlugin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_GetUpdatesManager(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFrameLayout_HasTopPlugin(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxFrameLayout_HideBarWindows(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_InverseVisibility(_obj: *mut ::libc::c_void,
                                           pBar: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnLButtonDown(_obj: *mut ::libc::c_void,
                                       event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnLButtonUp(_obj: *mut ::libc::c_void,
                                     event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnLDblClick(_obj: *mut ::libc::c_void,
                                     event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnMouseMove(_obj: *mut ::libc::c_void,
                                     event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnRButtonDown(_obj: *mut ::libc::c_void,
                                       event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnRButtonUp(_obj: *mut ::libc::c_void,
                                     event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_OnSize(_obj: *mut ::libc::c_void,
                                event: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_PopAllPlugins(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_PopPlugin(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_PushDefaultPlugins(_obj: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_PushPlugin(_obj: *mut ::libc::c_void,
                                    pPugin: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_RecalcLayout(_obj: *mut ::libc::c_void,
                                      repositionBarsNow: ::libc::c_int) -> ();
    pub fn wxFrameLayout_RedockBar(_obj: *mut ::libc::c_void,
                                   pBar: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int,
                                   w: ::libc::c_int, h: ::libc::c_int,
                                   pToPane: *mut ::libc::c_void,
                                   updateNow: ::libc::c_int) -> ::libc::c_int;
    pub fn wxFrameLayout_RefreshNow(_obj: *mut ::libc::c_void,
                                    recalcLayout: ::libc::c_int) -> ();
    pub fn wxFrameLayout_ReleaseEventsFromPane(_obj: *mut ::libc::c_void,
                                               fromPane: *mut ::libc::c_void)
     -> ();
    pub fn wxFrameLayout_ReleaseEventsFromPlugin(_obj: *mut ::libc::c_void,
                                                 pPlugin: *mut ::libc::c_void)
     -> ();
    pub fn wxFrameLayout_RemoveBar(_obj: *mut ::libc::c_void,
                                   pBar: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_RemovePlugin(_obj: *mut ::libc::c_void,
                                      pPlInfo: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_SetBarState(_obj: *mut ::libc::c_void,
                                     pBar: *mut ::libc::c_void,
                                     newStatem: ::libc::c_int,
                                     updateNow: ::libc::c_int) -> ();
    pub fn wxFrameLayout_SetFrameClient(_obj: *mut ::libc::c_void,
                                        pFrameClient: *mut ::libc::c_void)
     -> ();
    pub fn wxFrameLayout_SetMargins(_obj: *mut ::libc::c_void,
                                    top: ::libc::c_int, bottom: ::libc::c_int,
                                    left: ::libc::c_int, right: ::libc::c_int,
                                    paneMask: ::libc::c_int) -> ();
    pub fn wxFrameLayout_SetPaneBackground(_obj: *mut ::libc::c_void,
                                           colour: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_SetPaneProperties(_obj: *mut ::libc::c_void,
                                           props: *mut ::libc::c_void,
                                           paneMask: ::libc::c_int) -> ();
    pub fn wxFrameLayout_SetTopPlugin(_obj: *mut ::libc::c_void,
                                      pPlugin: *mut ::libc::c_void) -> ();
    pub fn wxFrameLayout_SetUpdatesManager(_obj: *mut ::libc::c_void,
                                           pUMgr: *mut ::libc::c_void) -> ();
    pub fn wxGauge_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                          _rng: ::libc::c_int, _lft: ::libc::c_int,
                          _top: ::libc::c_int, _wdt: ::libc::c_int,
                          _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGauge_GetBezelFace(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGauge_GetRange(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGauge_GetShadowWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGauge_GetValue(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGauge_SetBezelFace(_obj: *mut ::libc::c_void, w: ::libc::c_int)
     -> ();
    pub fn wxGauge_SetRange(_obj: *mut ::libc::c_void, r: ::libc::c_int)
     -> ();
    pub fn wxGauge_SetShadowWidth(_obj: *mut ::libc::c_void, w: ::libc::c_int)
     -> ();
    pub fn wxGauge_SetValue(_obj: *mut ::libc::c_void, pos: ::libc::c_int)
     -> ();
    pub fn wxGrid_AppendCols(_obj: *mut ::libc::c_void,
                             numCols: ::libc::c_int,
                             updateLabels: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_AppendRows(_obj: *mut ::libc::c_void,
                             numRows: ::libc::c_int,
                             updateLabels: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_AutoSize(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_AutoSizeColumn(_obj: *mut ::libc::c_void,
                                 col: ::libc::c_int, setAsMin: ::libc::c_int)
     -> ();
    pub fn wxGrid_AutoSizeColumns(_obj: *mut ::libc::c_void,
                                  setAsMin: ::libc::c_int) -> ();
    pub fn wxGrid_AutoSizeRow(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                              setAsMin: ::libc::c_int) -> ();
    pub fn wxGrid_AutoSizeRows(_obj: *mut ::libc::c_void,
                               setAsMin: ::libc::c_int) -> ();
    pub fn wxGrid_BeginBatch(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_BlockToDeviceRect(_obj: *mut ::libc::c_void,
                                    top: ::libc::c_int, left: ::libc::c_int,
                                    bottom: ::libc::c_int,
                                    right: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGrid_CanDragColSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_CanDragGridSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_CanDragRowSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_CanEnableCellControl(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_CellToRect(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                             col: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxGrid_ClearGrid(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_ClearSelection(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                         _lft: ::libc::c_int, _top: ::libc::c_int,
                         _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                         _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxGrid_CreateGrid(_obj: *mut ::libc::c_void, rows: ::libc::c_int,
                             cols: ::libc::c_int, selmode: ::libc::c_int)
     -> ();
    pub fn wxGrid_DeleteCols(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                             numCols: ::libc::c_int,
                             updateLabels: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_DeleteRows(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                             numRows: ::libc::c_int,
                             updateLabels: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_DisableCellEditControl(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DisableDragColSize(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DisableDragGridSize(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DisableDragRowSize(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DrawAllGridLines(_obj: *mut ::libc::c_void,
                                   dc: *mut ::libc::c_void,
                                   reg: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DrawCell(_obj: *mut ::libc::c_void, dc: *mut ::libc::c_void,
                           _row: ::libc::c_int, _col: ::libc::c_int) -> ();
    pub fn wxGrid_DrawCellBorder(_obj: *mut ::libc::c_void,
                                 dc: *mut ::libc::c_void, _row: ::libc::c_int,
                                 _col: ::libc::c_int) -> ();
    pub fn wxGrid_DrawCellHighlight(_obj: *mut ::libc::c_void,
                                    dc: *mut ::libc::c_void,
                                    attr: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DrawColLabel(_obj: *mut ::libc::c_void,
                               dc: *mut ::libc::c_void, col: ::libc::c_int)
     -> ();
    pub fn wxGrid_DrawColLabels(_obj: *mut ::libc::c_void,
                                dc: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DrawGridSpace(_obj: *mut ::libc::c_void,
                                dc: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DrawRowLabel(_obj: *mut ::libc::c_void,
                               dc: *mut ::libc::c_void, row: ::libc::c_int)
     -> ();
    pub fn wxGrid_DrawRowLabels(_obj: *mut ::libc::c_void,
                                dc: *mut ::libc::c_void) -> ();
    pub fn wxGrid_DrawTextRectangle(_obj: *mut ::libc::c_void,
                                    dc: *mut ::libc::c_void,
                                    txt: *mut ::libc::c_void,
                                    x: ::libc::c_int, y: ::libc::c_int,
                                    w: ::libc::c_int, h: ::libc::c_int,
                                    horizontalAlignment: ::libc::c_int,
                                    verticalAlignment: ::libc::c_int) -> ();
    pub fn wxGrid_EnableCellEditControl(_obj: *mut ::libc::c_void,
                                        enable: ::libc::c_int) -> ();
    pub fn wxGrid_EnableDragColSize(_obj: *mut ::libc::c_void,
                                    enable: ::libc::c_int) -> ();
    pub fn wxGrid_EnableDragGridSize(_obj: *mut ::libc::c_void,
                                     enable: ::libc::c_int) -> ();
    pub fn wxGrid_EnableDragRowSize(_obj: *mut ::libc::c_void,
                                    enable: ::libc::c_int) -> ();
    pub fn wxGrid_EnableEditing(_obj: *mut ::libc::c_void,
                                edit: ::libc::c_int) -> ();
    pub fn wxGrid_EnableGridLines(_obj: *mut ::libc::c_void,
                                  enable: ::libc::c_int) -> ();
    pub fn wxGrid_EndBatch(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetBatchCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetCellAlignment(_obj: *mut ::libc::c_void,
                                   row: ::libc::c_int, col: ::libc::c_int,
                                   horiz: *mut ::libc::c_int,
                                   vert: *mut ::libc::c_int) -> ();
    pub fn wxGrid_GetCellBackgroundColour(_obj: *mut ::libc::c_void,
                                          row: ::libc::c_int,
                                          col: ::libc::c_int,
                                          colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetCellEditor(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                                col: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxGrid_GetCellFont(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                              col: ::libc::c_int, font: *mut ::libc::c_void)
     -> ();
    pub fn wxGrid_GetCellHighlightColour(_obj: *mut ::libc::c_void,
                                         _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetCellRenderer(_obj: *mut ::libc::c_void,
                                  row: ::libc::c_int, col: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetCellTextColour(_obj: *mut ::libc::c_void,
                                    row: ::libc::c_int, col: ::libc::c_int,
                                    colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetCellValue(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                               col: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxGrid_GetColLabelAlignment(_obj: *mut ::libc::c_void,
                                       horiz: *mut ::libc::c_int,
                                       vert: *mut ::libc::c_int) -> ();
    pub fn wxGrid_GetColLabelSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetColLabelValue(_obj: *mut ::libc::c_void,
                                   col: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxGrid_GetColSize(_obj: *mut ::libc::c_void, col: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_GetDefaultCellAlignment(_obj: *mut ::libc::c_void,
                                          horiz: *mut ::libc::c_int,
                                          vert: *mut ::libc::c_int) -> ();
    pub fn wxGrid_GetDefaultCellBackgroundColour(_obj: *mut ::libc::c_void,
                                                 _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxGrid_GetDefaultCellFont(_obj: *mut ::libc::c_void,
                                     _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetDefaultCellTextColour(_obj: *mut ::libc::c_void,
                                           _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetDefaultColLabelSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_GetDefaultColSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_GetDefaultEditor(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetDefaultEditorForCell(_obj: *mut ::libc::c_void,
                                          row: ::libc::c_int,
                                          col: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetDefaultEditorForType(_obj: *mut ::libc::c_void,
                                          typeName: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetDefaultRenderer(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetDefaultRendererForCell(_obj: *mut ::libc::c_void,
                                            row: ::libc::c_int,
                                            col: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetDefaultRendererForType(_obj: *mut ::libc::c_void,
                                            typeName: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGrid_GetDefaultRowLabelSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_GetDefaultRowSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_GetGridCursorCol(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_GetGridCursorRow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_GetGridLineColour(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetLabelBackgroundColour(_obj: *mut ::libc::c_void,
                                           _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetLabelFont(_obj: *mut ::libc::c_void,
                               _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetLabelTextColour(_obj: *mut ::libc::c_void,
                                     _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetNumberCols(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetNumberRows(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetRowLabelAlignment(_obj: *mut ::libc::c_void,
                                       horiz: *mut ::libc::c_int,
                                       vert: *mut ::libc::c_int) -> ();
    pub fn wxGrid_GetRowLabelSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetRowLabelValue(_obj: *mut ::libc::c_void,
                                   row: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxGrid_GetRowSize(_obj: *mut ::libc::c_void, row: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_GetSelectionBackground(_obj: *mut ::libc::c_void,
                                         _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetSelectionForeground(_obj: *mut ::libc::c_void,
                                         _ref: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetTable(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxGrid_GetTextBoxSize(_obj: *mut ::libc::c_void,
                                 dc: *mut ::libc::c_void,
                                 count: ::libc::c_int,
                                 lines: *mut *mut ::libc::c_char,
                                 _w: *mut ::libc::c_void,
                                 _h: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GridLinesEnabled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_HideCellEditControl(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_InsertCols(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                             numCols: ::libc::c_int,
                             updateLabels: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_InsertRows(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                             numRows: ::libc::c_int,
                             updateLabels: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_IsCellEditControlEnabled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_IsCellEditControlShown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_IsCurrentCellReadOnly(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_IsEditable(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_IsInSelection(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                                col: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_IsReadOnly(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                             col: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_IsSelection(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_IsVisible(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                            col: ::libc::c_int,
                            wholeCellVisible: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_MakeCellVisible(_obj: *mut ::libc::c_void,
                                  row: ::libc::c_int, col: ::libc::c_int)
     -> ();
    pub fn wxGrid_MoveCursorDown(_obj: *mut ::libc::c_void,
                                 expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorDownBlock(_obj: *mut ::libc::c_void,
                                      expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorLeft(_obj: *mut ::libc::c_void,
                                 expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorLeftBlock(_obj: *mut ::libc::c_void,
                                      expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorRight(_obj: *mut ::libc::c_void,
                                  expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorRightBlock(_obj: *mut ::libc::c_void,
                                       expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorUp(_obj: *mut ::libc::c_void,
                               expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MoveCursorUpBlock(_obj: *mut ::libc::c_void,
                                    expandSelection: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_MovePageDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_MovePageUp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_ProcessTableMessage(_obj: *mut ::libc::c_void,
                                      evt: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGrid_RegisterDataType(_obj: *mut ::libc::c_void,
                                   typeName: *mut ::libc::c_void,
                                   renderer: *mut ::libc::c_void,
                                   editor: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SaveEditControlValue(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SelectAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SelectBlock(_obj: *mut ::libc::c_void,
                              topRow: ::libc::c_int, leftCol: ::libc::c_int,
                              bottomRow: ::libc::c_int,
                              rightCol: ::libc::c_int,
                              addToSelected: ::libc::c_int) -> ();
    pub fn wxGrid_SelectCol(_obj: *mut ::libc::c_void, col: ::libc::c_int,
                            addToSelected: ::libc::c_int) -> ();
    pub fn wxGrid_SelectRow(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                            addToSelected: ::libc::c_int) -> ();
    pub fn wxGrid_SetCellAlignment(_obj: *mut ::libc::c_void,
                                   row: ::libc::c_int, col: ::libc::c_int,
                                   horiz: ::libc::c_int, vert: ::libc::c_int)
     -> ();
    pub fn wxGrid_SetCellBackgroundColour(_obj: *mut ::libc::c_void,
                                          row: ::libc::c_int,
                                          col: ::libc::c_int,
                                          colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetCellEditor(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                                col: ::libc::c_int,
                                editor: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetCellFont(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                              col: ::libc::c_int, font: *mut ::libc::c_void)
     -> ();
    pub fn wxGrid_SetCellHighlightColour(_obj: *mut ::libc::c_void,
                                         col: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetCellRenderer(_obj: *mut ::libc::c_void,
                                  row: ::libc::c_int, col: ::libc::c_int,
                                  renderer: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetCellTextColour(_obj: *mut ::libc::c_void,
                                    row: ::libc::c_int, col: ::libc::c_int,
                                    colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetCellValue(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                               col: ::libc::c_int, s: *mut ::libc::c_void)
     -> ();
    pub fn wxGrid_SetColAttr(_obj: *mut ::libc::c_void, col: ::libc::c_int,
                             attr: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetColFormatBool(_obj: *mut ::libc::c_void,
                                   col: ::libc::c_int) -> ();
    pub fn wxGrid_SetColFormatCustom(_obj: *mut ::libc::c_void,
                                     col: ::libc::c_int,
                                     typeName: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetColFormatFloat(_obj: *mut ::libc::c_void,
                                    col: ::libc::c_int, width: ::libc::c_int,
                                    precision: ::libc::c_int) -> ();
    pub fn wxGrid_SetColFormatNumber(_obj: *mut ::libc::c_void,
                                     col: ::libc::c_int) -> ();
    pub fn wxGrid_SetColLabelAlignment(_obj: *mut ::libc::c_void,
                                       horiz: ::libc::c_int,
                                       vert: ::libc::c_int) -> ();
    pub fn wxGrid_SetColLabelSize(_obj: *mut ::libc::c_void,
                                  height: ::libc::c_int) -> ();
    pub fn wxGrid_SetColLabelValue(_obj: *mut ::libc::c_void,
                                   col: ::libc::c_int,
                                   label: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetColMinimalWidth(_obj: *mut ::libc::c_void,
                                     col: ::libc::c_int, width: ::libc::c_int)
     -> ();
    pub fn wxGrid_SetColSize(_obj: *mut ::libc::c_void, col: ::libc::c_int,
                             width: ::libc::c_int) -> ();
    pub fn wxGrid_SetDefaultCellAlignment(_obj: *mut ::libc::c_void,
                                          horiz: ::libc::c_int,
                                          vert: ::libc::c_int) -> ();
    pub fn wxGrid_SetDefaultCellBackgroundColour(_obj: *mut ::libc::c_void,
                                                 colour: *mut ::libc::c_void)
     -> ();
    pub fn wxGrid_SetDefaultCellFont(_obj: *mut ::libc::c_void,
                                     font: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetDefaultCellTextColour(_obj: *mut ::libc::c_void,
                                           colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetDefaultColSize(_obj: *mut ::libc::c_void,
                                    width: ::libc::c_int,
                                    resizeExistingCols: ::libc::c_int) -> ();
    pub fn wxGrid_SetDefaultEditor(_obj: *mut ::libc::c_void,
                                   editor: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetDefaultRenderer(_obj: *mut ::libc::c_void,
                                     renderer: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetDefaultRowSize(_obj: *mut ::libc::c_void,
                                    height: ::libc::c_int,
                                    resizeExistingRows: ::libc::c_int) -> ();
    pub fn wxGrid_SetGridCursor(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                                col: ::libc::c_int) -> ();
    pub fn wxGrid_SetGridLineColour(_obj: *mut ::libc::c_void,
                                    col: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetLabelBackgroundColour(_obj: *mut ::libc::c_void,
                                           colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetLabelFont(_obj: *mut ::libc::c_void,
                               font: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetLabelTextColour(_obj: *mut ::libc::c_void,
                                     colour: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetMargins(_obj: *mut ::libc::c_void,
                             extraWidth: ::libc::c_int,
                             extraHeight: ::libc::c_int) -> ();
    pub fn wxGrid_SetReadOnly(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                              col: ::libc::c_int, isReadOnly: ::libc::c_int)
     -> ();
    pub fn wxGrid_SetRowAttr(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                             attr: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetRowLabelAlignment(_obj: *mut ::libc::c_void,
                                       horiz: ::libc::c_int,
                                       vert: ::libc::c_int) -> ();
    pub fn wxGrid_SetRowLabelSize(_obj: *mut ::libc::c_void,
                                  width: ::libc::c_int) -> ();
    pub fn wxGrid_SetRowLabelValue(_obj: *mut ::libc::c_void,
                                   row: ::libc::c_int,
                                   label: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetRowMinimalHeight(_obj: *mut ::libc::c_void,
                                      row: ::libc::c_int,
                                      width: ::libc::c_int) -> ();
    pub fn wxGrid_SetRowSize(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                             height: ::libc::c_int) -> ();
    pub fn wxGrid_SetSelectionBackground(_obj: *mut ::libc::c_void,
                                         c: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetSelectionForeground(_obj: *mut ::libc::c_void,
                                         c: *mut ::libc::c_void) -> ();
    pub fn wxGrid_SetSelectionMode(_obj: *mut ::libc::c_void,
                                   selmode: ::libc::c_int) -> ();
    pub fn wxGrid_SetTable(_obj: *mut ::libc::c_void,
                           table: *mut ::libc::c_void,
                           takeOwnership: ::libc::c_int,
                           selmode: ::libc::c_int) -> ::libc::c_int;
    pub fn wxGrid_ShowCellEditControl(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGrid_StringToLines(_obj: *mut ::libc::c_void,
                                value: *mut ::libc::c_void,
                                lines: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_XToCol(_obj: *mut ::libc::c_void, x: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_XToEdgeOfCol(_obj: *mut ::libc::c_void, x: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_XYToCell(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                           y: ::libc::c_int, row: *mut ::libc::c_int,
                           col: *mut ::libc::c_int) -> ();
    pub fn wxGrid_YToEdgeOfRow(_obj: *mut ::libc::c_void, y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_YToRow(_obj: *mut ::libc::c_void, y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGrid_GetSelectedCells(_obj: *mut ::libc::c_void,
                                   _arr: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetSelectionBlockTopLeft(_obj: *mut ::libc::c_void,
                                           _arr: *mut ::libc::c_void) -> ();
    pub fn wxGrid_GetSelectionBlockBottomRight(_obj: *mut ::libc::c_void,
                                               _arr: *mut ::libc::c_void)
     -> ();
    pub fn wxGrid_GetSelectedRows(_obj: *mut ::libc::c_void,
                                  _arr: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetSelectedCols(_obj: *mut ::libc::c_void,
                                  _arr: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGrid_GetCellSize(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                              col: ::libc::c_int, srow: *mut ::libc::c_int,
                              scol: *mut ::libc::c_int) -> ();
    pub fn wxGrid_SetCellSize(_obj: *mut ::libc::c_void, row: ::libc::c_int,
                              col: ::libc::c_int, srow: ::libc::c_int,
                              scol: ::libc::c_int) -> ();
    pub fn wxGridCellAttr_Ctor() -> *mut ::libc::c_void;
    pub fn wxGridCellAttr_DecRef(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_GetAlignment(_obj: *mut ::libc::c_void,
                                       hAlign: *mut ::libc::c_int,
                                       vAlign: *mut ::libc::c_int) -> ();
    pub fn wxGridCellAttr_GetBackgroundColour(_obj: *mut ::libc::c_void,
                                              _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxGridCellAttr_GetEditor(_obj: *mut ::libc::c_void,
                                    grid: *mut ::libc::c_void,
                                    row: ::libc::c_int, col: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGridCellAttr_GetFont(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_GetRenderer(_obj: *mut ::libc::c_void,
                                      grid: *mut ::libc::c_void,
                                      row: ::libc::c_int, col: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGridCellAttr_GetTextColour(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_HasAlignment(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellAttr_HasBackgroundColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellAttr_HasEditor(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellAttr_HasFont(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridCellAttr_HasRenderer(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellAttr_HasTextColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellAttr_IncRef(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_IsReadOnly(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellAttr_SetAlignment(_obj: *mut ::libc::c_void,
                                       hAlign: ::libc::c_int,
                                       vAlign: ::libc::c_int) -> ();
    pub fn wxGridCellAttr_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                              colBack: *mut ::libc::c_void)
     -> ();
    pub fn wxGridCellAttr_SetDefAttr(_obj: *mut ::libc::c_void,
                                     defAttr: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_SetEditor(_obj: *mut ::libc::c_void,
                                    editor: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_SetFont(_obj: *mut ::libc::c_void,
                                  font: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_SetReadOnly(_obj: *mut ::libc::c_void,
                                      isReadOnly: ::libc::c_int) -> ();
    pub fn wxGridCellAttr_SetRenderer(_obj: *mut ::libc::c_void,
                                      renderer: *mut ::libc::c_void) -> ();
    pub fn wxGridCellAttr_SetTextColour(_obj: *mut ::libc::c_void,
                                        colText: *mut ::libc::c_void) -> ();
    pub fn wxGridCellBoolEditor_Ctor() -> *mut ::libc::c_void;
    pub fn wxGridCellChoiceEditor_Ctor(count: ::libc::c_int,
                                       choices: *mut *mut ::libc::c_char,
                                       allowOthers: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGridCellCoordsArray_Create() -> *mut ::libc::c_void;
    pub fn wxGridCellCoordsArray_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridCellCoordsArray_GetCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellCoordsArray_Item(_obj: *mut ::libc::c_void,
                                      _idx: ::libc::c_int,
                                      _c: *mut ::libc::c_int,
                                      _r: *mut ::libc::c_int) -> ();
    pub fn wxGridCellEditor_BeginEdit(_obj: *mut ::libc::c_void,
                                      row: ::libc::c_int, col: ::libc::c_int,
                                      grid: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_Create(_obj: *mut ::libc::c_void,
                                   parent: *mut ::libc::c_void,
                                   id: ::libc::c_int,
                                   evtHandler: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_Destroy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_EndEdit(_obj: *mut ::libc::c_void,
                                    row: ::libc::c_int, col: ::libc::c_int,
                                    grid: *mut ::libc::c_void,
                                    oldStr: *mut ::libc::c_void,
                                    newStr: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellEditor_GetControl(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGridCellEditor_HandleReturn(_obj: *mut ::libc::c_void,
                                         event: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_IsAcceptedKey(_obj: *mut ::libc::c_void,
                                          event: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellEditor_IsCreated(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridCellEditor_PaintBackground(_obj: *mut ::libc::c_void,
                                            dc: *mut ::libc::c_void,
                                            x: ::libc::c_int,
                                            y: ::libc::c_int,
                                            w: ::libc::c_int,
                                            h: ::libc::c_int,
                                            attr: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_Reset(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_SetControl(_obj: *mut ::libc::c_void,
                                       control: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_SetParameters(_obj: *mut ::libc::c_void,
                                          params: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_SetSize(_obj: *mut ::libc::c_void,
                                    x: ::libc::c_int, y: ::libc::c_int,
                                    w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxGridCellEditor_Show(_obj: *mut ::libc::c_void,
                                 show: ::libc::c_int,
                                 attr: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_StartingClick(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridCellEditor_StartingKey(_obj: *mut ::libc::c_void,
                                        event: *mut ::libc::c_void) -> ();
    pub fn wxGridCellFloatEditor_Ctor(width: ::libc::c_int,
                                      precision: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGridCellNumberEditor_Ctor(min: ::libc::c_int, max: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGridCellNumberRenderer_Ctor() -> *mut ::libc::c_void;
    pub fn wxGridCellAutoWrapStringRenderer_Ctor() -> *mut ::libc::c_void;
    pub fn wxGridCellTextEditor_Ctor() -> *mut ::libc::c_void;
    pub fn wxGridEditorCreatedEvent_GetCol(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridEditorCreatedEvent_GetControl(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGridEditorCreatedEvent_GetRow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridEditorCreatedEvent_SetCol(_obj: *mut ::libc::c_void,
                                           col: ::libc::c_int) -> ();
    pub fn wxGridEditorCreatedEvent_SetControl(_obj: *mut ::libc::c_void,
                                               ctrl: *mut ::libc::c_void)
     -> ();
    pub fn wxGridEditorCreatedEvent_SetRow(_obj: *mut ::libc::c_void,
                                           row: ::libc::c_int) -> ();
    pub fn wxGridEvent_AltDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridEvent_ControlDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridEvent_GetCol(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGridEvent_GetRow(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridEvent_MetaDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridEvent_Selecting(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridEvent_ShiftDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_GetTopLeftCoords(_obj: *mut ::libc::c_void,
                                                   col: *mut ::libc::c_void,
                                                   row: *mut ::libc::c_void)
     -> ();
    pub fn wxGridRangeSelectEvent_GetBottomRightCoords(_obj:
                                                           *mut ::libc::c_void,
                                                       col:
                                                           *mut ::libc::c_void,
                                                       row:
                                                           *mut ::libc::c_void)
     -> ();
    pub fn wxGridRangeSelectEvent_GetTopRow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_GetBottomRow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_GetLeftCol(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_GetRightCol(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_Selecting(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_ControlDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_MetaDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_ShiftDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridRangeSelectEvent_AltDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridSizeEvent_GetRowOrCol(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridSizeEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGridSizeEvent_ControlDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridSizeEvent_MetaDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridSizeEvent_ShiftDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridSizeEvent_AltDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGridSizer_CalcMin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGridSizer_Create(rows: ::libc::c_int, cols: ::libc::c_int,
                              vgap: ::libc::c_int, hgap: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxGridSizer_GetCols(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridSizer_GetHGap(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridSizer_GetRows(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridSizer_GetVGap(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGridSizer_RecalcSizes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxGridSizer_SetCols(_obj: *mut ::libc::c_void, cols: ::libc::c_int)
     -> ();
    pub fn wxGridSizer_SetHGap(_obj: *mut ::libc::c_void, gap: ::libc::c_int)
     -> ();
    pub fn wxGridSizer_SetRows(_obj: *mut ::libc::c_void, rows: ::libc::c_int)
     -> ();
    pub fn wxGridSizer_SetVGap(_obj: *mut ::libc::c_void, gap: ::libc::c_int)
     -> ();
    pub fn wxHelpControllerHelpProvider_Create(ctr: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpControllerHelpProvider_GetHelpController(_obj:
                                                              *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpControllerHelpProvider_SetHelpController(_obj:
                                                              *mut ::libc::c_void,
                                                          hc:
                                                              *mut ::libc::c_void)
     -> ();
    pub fn wxHelpEvent_GetLink(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpEvent_GetTarget(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpEvent_SetLink(_obj: *mut ::libc::c_void,
                               link: *mut ::libc::c_void) -> ();
    pub fn wxHelpEvent_SetPosition(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxHelpEvent_SetTarget(_obj: *mut ::libc::c_void,
                                 target: *mut ::libc::c_void) -> ();
    pub fn wxHelpProvider_AddHelp(_obj: *mut ::libc::c_void,
                                  window: *mut ::libc::c_void,
                                  text: *mut ::libc::c_void) -> ();
    pub fn wxHelpProvider_AddHelpById(_obj: *mut ::libc::c_void,
                                      id: ::libc::c_int,
                                      text: *mut ::libc::c_void) -> ();
    pub fn wxHelpProvider_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxHelpProvider_Get() -> *mut ::libc::c_void;
    pub fn wxHelpProvider_GetHelp(_obj: *mut ::libc::c_void,
                                  window: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpProvider_RemoveHelp(_obj: *mut ::libc::c_void,
                                     window: *mut ::libc::c_void) -> ();
    pub fn wxHelpProvider_Set(helpProvider: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHelpProvider_ShowHelp(_obj: *mut ::libc::c_void,
                                   window: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_AddBook(_obj: *mut ::libc::c_void,
                                        book: *mut ::libc::c_void,
                                        show_wait_msg: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_Create(_style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxHtmlHelpController_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxHtmlHelpController_Display(_obj: *mut ::libc::c_void,
                                        x: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_DisplayBlock(_obj: *mut ::libc::c_void,
                                             blockNo: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_DisplayContents(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_DisplayIndex(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_DisplayNumber(_obj: *mut ::libc::c_void,
                                              id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_DisplaySection(_obj: *mut ::libc::c_void,
                                               section: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_DisplaySectionNumber(_obj:
                                                         *mut ::libc::c_void,
                                                     sectionNo: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_GetFrame(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlHelpController_GetFrameParameters(_obj: *mut ::libc::c_void,
                                                   title: *mut ::libc::c_void,
                                                   width: *mut ::libc::c_int,
                                                   height: *mut ::libc::c_int,
                                                   pos_x: *mut ::libc::c_int,
                                                   pos_y: *mut ::libc::c_int,
                                                   newFrameEachTime:
                                                       *mut ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxHtmlHelpController_Initialize(_obj: *mut ::libc::c_void,
                                           file: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_KeywordSearch(_obj: *mut ::libc::c_void,
                                              keyword: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_LoadFile(_obj: *mut ::libc::c_void,
                                         file: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_Quit(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlHelpController_ReadCustomization(_obj: *mut ::libc::c_void,
                                                  cfg: *mut ::libc::c_void,
                                                  path: *mut ::libc::c_void)
     -> ();
    pub fn wxHtmlHelpController_SetFrameParameters(_obj: *mut ::libc::c_void,
                                                   title: *mut ::libc::c_void,
                                                   width: ::libc::c_int,
                                                   height: ::libc::c_int,
                                                   pos_x: ::libc::c_int,
                                                   pos_y: ::libc::c_int,
                                                   newFrameEachTime:
                                                       ::libc::c_int) -> ();
    pub fn wxHtmlHelpController_SetTempDir(_obj: *mut ::libc::c_void,
                                           path: *mut ::libc::c_void) -> ();
    pub fn wxHtmlHelpController_SetTitleFormat(_obj: *mut ::libc::c_void,
                                               format: *mut ::libc::c_void)
     -> ();
    pub fn wxHtmlHelpController_SetViewer(_obj: *mut ::libc::c_void,
                                          viewer: *mut ::libc::c_void,
                                          flags: ::libc::c_int) -> ();
    pub fn wxHtmlHelpController_UseConfig(_obj: *mut ::libc::c_void,
                                          config: *mut ::libc::c_void,
                                          rootpath: *mut ::libc::c_void)
     -> ();
    pub fn wxHtmlHelpController_WriteCustomization(_obj: *mut ::libc::c_void,
                                                   cfg: *mut ::libc::c_void,
                                                   path: *mut ::libc::c_void)
     -> ();
    pub fn wxIcon_Assign(_obj: *mut ::libc::c_void,
                         other: *mut ::libc::c_void) -> ();
    pub fn wxIcon_CopyFromBitmap(_obj: *mut ::libc::c_void,
                                 bmp: *mut ::libc::c_void) -> ();
    pub fn wxIcon_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxIcon_CreateLoad(name: *mut ::libc::c_void, _type: ::libc::c_long,
                             width: ::libc::c_int, height: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxIcon_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxIcon_FromRaw(data: *mut ::libc::c_void, width: ::libc::c_int,
                          height: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxIcon_FromXPM(data: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxIcon_GetDepth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxIcon_GetHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxIcon_GetWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxIcon_IsEqual(_obj: *mut ::libc::c_void,
                          other: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxIcon_Load(_obj: *mut ::libc::c_void, name: *mut ::libc::c_void,
                       _type: ::libc::c_long, width: ::libc::c_int,
                       height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxIcon_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxIcon_SetDepth(_obj: *mut ::libc::c_void, depth: ::libc::c_int)
     -> ();
    pub fn wxIcon_SetHeight(_obj: *mut ::libc::c_void, height: ::libc::c_int)
     -> ();
    pub fn wxIcon_SetWidth(_obj: *mut ::libc::c_void, width: ::libc::c_int)
     -> ();
    pub fn wxIconBundle_AddIcon(_obj: *mut ::libc::c_void,
                                icon: *mut ::libc::c_void) -> ();
    pub fn wxIconBundle_AddIconFromFile(_obj: *mut ::libc::c_void,
                                        file: *mut ::libc::c_void,
                                        _type: ::libc::c_int) -> ();
    pub fn wxIconBundle_Assign(_obj: *mut ::libc::c_void,
                               _ref: *mut ::libc::c_void) -> ();
    pub fn wxIconBundle_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxIconBundle_CreateFromFile(file: *mut ::libc::c_void,
                                       _type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxIconBundle_CreateFromIcon(icon: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxIconBundle_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxIconBundle_GetIcon(_obj: *mut ::libc::c_void, w: ::libc::c_int,
                                h: ::libc::c_int, _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxIdleEvent_CopyObject(_obj: *mut ::libc::c_void,
                                  object_dest: *mut ::libc::c_void) -> ();
    pub fn wxIdleEvent_MoreRequested(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIdleEvent_RequestMore(_obj: *mut ::libc::c_void,
                                   needMore: ::libc::c_int) -> ();
    pub fn wxImage_CanRead(name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_ConvertToBitmap(_obj: *mut ::libc::c_void,
                                   bitmap: *mut ::libc::c_void) -> ();
    pub fn wxImage_ConvertToByteString(_obj: *mut ::libc::c_void,
                                       _type: ::libc::c_int,
                                       data: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn wxImage_ConvertToLazyByteString(_obj: *mut ::libc::c_void,
                                           _type: ::libc::c_int,
                                           data: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn wxImage_CountColours(_obj: *mut ::libc::c_void,
                                stopafter: ::libc::c_int) -> ::libc::c_int;
    pub fn wxImage_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxImage_CreateFromBitmap(bitmap: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxImage_CreateFromByteString(data: *mut *mut ::libc::c_char,
                                        length: ::libc::c_int,
                                        _type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxImage_CreateFromLazyByteString(data: *mut *mut ::libc::c_char,
                                            length: ::libc::c_int,
                                            _type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxImage_CreateFromData(width: ::libc::c_int, height: ::libc::c_int,
                                  data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxImage_CreateFromFile(name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxImage_CreateSized(width: ::libc::c_int, height: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxImage_Destroy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxImage_GetBlue(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                           y: ::libc::c_int) -> ::libc::c_char;
    pub fn wxImage_GetData(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxImage_GetGreen(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                            y: ::libc::c_int) -> ::libc::c_char;
    pub fn wxImage_GetHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_GetMaskBlue(_obj: *mut ::libc::c_void) -> ::libc::c_char;
    pub fn wxImage_GetMaskGreen(_obj: *mut ::libc::c_void) -> ::libc::c_char;
    pub fn wxImage_GetMaskRed(_obj: *mut ::libc::c_void) -> ::libc::c_char;
    pub fn wxImage_GetRed(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                          y: ::libc::c_int) -> ::libc::c_char;
    pub fn wxImage_GetSubImage(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                               y: ::libc::c_int, w: ::libc::c_int,
                               h: ::libc::c_int, image: *mut ::libc::c_void)
     -> ();
    pub fn wxImage_GetWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_HasMask(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_GetOption(_obj: *mut ::libc::c_void,
                             name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxImage_GetOptionInt(_obj: *mut ::libc::c_void,
                                name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_HasOption(_obj: *mut ::libc::c_void,
                             name: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_Initialize(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                              height: ::libc::c_int) -> ();
    pub fn wxImage_InitializeFromData(_obj: *mut ::libc::c_void,
                                      width: ::libc::c_int,
                                      height: ::libc::c_int,
                                      data: *mut ::libc::c_void) -> ();
    pub fn wxImage_LoadFile(_obj: *mut ::libc::c_void,
                            name: *mut ::libc::c_void, _type: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxImage_Mirror(_obj: *mut ::libc::c_void,
                          horizontally: ::libc::c_int,
                          image: *mut ::libc::c_void) -> ();
    pub fn wxImage_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImage_Paste(_obj: *mut ::libc::c_void,
                         image: *mut ::libc::c_void, x: ::libc::c_int,
                         y: ::libc::c_int) -> ();
    pub fn wxImage_Replace(_obj: *mut ::libc::c_void, r1: uint8_t,
                           g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t,
                           b2: uint8_t) -> ();
    pub fn wxImage_Rescale(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                           height: ::libc::c_int) -> ();
    pub fn wxImage_Rotate(_obj: *mut ::libc::c_void, angle: ::libc::c_double,
                          c_x: ::libc::c_int, c_y: ::libc::c_int,
                          interpolating: ::libc::c_int,
                          offset_after_rotation: *mut ::libc::c_void,
                          image: *mut ::libc::c_void) -> ();
    pub fn wxImage_Rotate90(_obj: *mut ::libc::c_void,
                            clockwise: ::libc::c_int,
                            image: *mut ::libc::c_void) -> ();
    pub fn wxImage_SaveFile(_obj: *mut ::libc::c_void,
                            name: *mut ::libc::c_void, _type: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxImage_Scale(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                         height: ::libc::c_int, image: *mut ::libc::c_void)
     -> ();
    pub fn wxImage_SetData(_obj: *mut ::libc::c_void,
                           data: *mut ::libc::c_void) -> ();
    pub fn wxImage_SetDataAndSize(_obj: *mut ::libc::c_void,
                                  data: *mut ::libc::c_void,
                                  new_width: ::libc::c_int,
                                  new_height: ::libc::c_int) -> ();
    pub fn wxImage_SetMask(_obj: *mut ::libc::c_void, mask: ::libc::c_int)
     -> ();
    pub fn wxImage_SetMaskColour(_obj: *mut ::libc::c_void, r: uint8_t,
                                 g: uint8_t, b: uint8_t) -> ();
    pub fn wxImage_SetOption(_obj: *mut ::libc::c_void,
                             name: *mut ::libc::c_void,
                             value: *mut ::libc::c_void) -> ();
    pub fn wxImage_SetOptionInt(_obj: *mut ::libc::c_void,
                                name: *mut ::libc::c_void,
                                value: ::libc::c_int) -> ();
    pub fn wxImage_SetRGB(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                          y: ::libc::c_int, r: uint8_t, g: uint8_t,
                          b: uint8_t) -> ();
    pub fn wxImageList_AddBitmap(_obj: *mut ::libc::c_void,
                                 bitmap: *mut ::libc::c_void,
                                 mask: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImageList_AddIcon(_obj: *mut ::libc::c_void,
                               icon: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImageList_AddMasked(_obj: *mut ::libc::c_void,
                                 bitmap: *mut ::libc::c_void,
                                 maskColour: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxImageList_Create(width: ::libc::c_int, height: ::libc::c_int,
                              mask: ::libc::c_int,
                              initialCount: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxImageList_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxImageList_Draw(_obj: *mut ::libc::c_void, index: ::libc::c_int,
                            dc: *mut ::libc::c_void, x: ::libc::c_int,
                            y: ::libc::c_int, flags: ::libc::c_int,
                            solidBackground: ::libc::c_int) -> ::libc::c_int;
    pub fn wxImageList_GetImageCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxImageList_GetSize(_obj: *mut ::libc::c_void,
                               index: ::libc::c_int,
                               width: *mut ::libc::c_int,
                               height: *mut ::libc::c_int) -> ();
    pub fn wxImageList_Remove(_obj: *mut ::libc::c_void, index: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxImageList_RemoveAll(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImageList_Replace(_obj: *mut ::libc::c_void,
                               index: ::libc::c_int,
                               bitmap: *mut ::libc::c_void,
                               mask: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxImageList_ReplaceIcon(_obj: *mut ::libc::c_void,
                                   index: ::libc::c_int,
                                   icon: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_Above(_obj: *mut ::libc::c_void,
                                              sibling: *mut ::libc::c_void,
                                              marg: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_Absolute(_obj: *mut ::libc::c_void,
                                                 val: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_AsIs(_obj: *mut ::libc::c_void) -> ();
    pub fn wxIndividualLayoutConstraint_Below(_obj: *mut ::libc::c_void,
                                              sibling: *mut ::libc::c_void,
                                              marg: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_GetDone(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetEdge(_obj: *mut ::libc::c_void,
                                                which: ::libc::c_int,
                                                thisWin: *mut ::libc::c_void,
                                                other: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetMargin(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetMyEdge(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetOtherEdge(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetOtherWindow(_obj:
                                                           *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxIndividualLayoutConstraint_GetPercent(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetRelationship(_obj:
                                                            *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_GetValue(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_LeftOf(_obj: *mut ::libc::c_void,
                                               sibling: *mut ::libc::c_void,
                                               marg: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_PercentOf(_obj: *mut ::libc::c_void,
                                                  otherW: *mut ::libc::c_void,
                                                  wh: ::libc::c_int,
                                                  per: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_ResetIfWin(_obj: *mut ::libc::c_void,
                                                   otherW:
                                                       *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_RightOf(_obj: *mut ::libc::c_void,
                                                sibling: *mut ::libc::c_void,
                                                marg: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_SameAs(_obj: *mut ::libc::c_void,
                                               otherW: *mut ::libc::c_void,
                                               edge: ::libc::c_int,
                                               marg: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_SatisfyConstraint(_obj:
                                                              *mut ::libc::c_void,
                                                          constraints:
                                                              *mut ::libc::c_void,
                                                          win:
                                                              *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxIndividualLayoutConstraint_Set(_obj: *mut ::libc::c_void,
                                            rel: ::libc::c_int,
                                            otherW: *mut ::libc::c_void,
                                            otherE: ::libc::c_int,
                                            val: ::libc::c_int,
                                            marg: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_SetDone(_obj: *mut ::libc::c_void,
                                                d: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_SetEdge(_obj: *mut ::libc::c_void,
                                                which: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_SetMargin(_obj: *mut ::libc::c_void,
                                                  m: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_SetRelationship(_obj:
                                                            *mut ::libc::c_void,
                                                        r: ::libc::c_int)
     -> ();
    pub fn wxIndividualLayoutConstraint_SetValue(_obj: *mut ::libc::c_void,
                                                 v: ::libc::c_int) -> ();
    pub fn wxIndividualLayoutConstraint_Unconstrained(_obj:
                                                          *mut ::libc::c_void)
     -> ();
    pub fn wxInputStream_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxInputStream_Eof(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxInputStream_GetC(_obj: *mut ::libc::c_void) -> ::libc::c_char;
    pub fn wxInputStream_LastRead(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxInputStream_Peek(_obj: *mut ::libc::c_void) -> ::libc::c_char;
    pub fn wxInputStream_Read(_obj: *mut ::libc::c_void,
                              buffer: *mut ::libc::c_void,
                              size: ::libc::c_int) -> ();
    pub fn wxInputStream_SeekI(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                               mode: ::libc::c_int) -> ::libc::c_int;
    pub fn wxInputStream_Tell(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxInputStream_UngetBuffer(_obj: *mut ::libc::c_void,
                                     buffer: *mut ::libc::c_void,
                                     size: ::libc::c_int) -> ::libc::c_int;
    pub fn wxInputStream_Ungetch(_obj: *mut ::libc::c_void, c: ::libc::c_char)
     -> ::libc::c_int;
    pub fn wxJoystick_Create(joystick: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxJoystick_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxJoystick_GetButtonState(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetManufacturerId(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetMaxAxes(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetMaxButtons(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetMovementThreshold(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetNumberAxes(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetNumberButtons(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetNumberJoysticks(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetPOVCTSPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetPOVPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetPollingMax(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetPollingMin(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxJoystick_GetProductId(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetProductName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxJoystick_GetRudderMax(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetRudderMin(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetRudderPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetUMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetUMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetUPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetVMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetVMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetVPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_GetXMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetXMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetYMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetYMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetZMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetZMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_GetZPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_HasPOV(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_HasPOV4Dir(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_HasPOVCTS(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_HasRudder(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_HasU(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_HasV(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_HasZ(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystick_ReleaseCapture(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystick_SetCapture(_obj: *mut ::libc::c_void,
                                 win: *mut ::libc::c_void,
                                 pollingFreq: ::libc::c_int) -> ::libc::c_int;
    pub fn wxJoystick_SetMovementThreshold(_obj: *mut ::libc::c_void,
                                           threshold: ::libc::c_int) -> ();
    pub fn wxJoystickEvent_ButtonDown(_obj: *mut ::libc::c_void,
                                      but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxJoystickEvent_ButtonIsDown(_obj: *mut ::libc::c_void,
                                        but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxJoystickEvent_ButtonUp(_obj: *mut ::libc::c_void,
                                    but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxJoystickEvent_CopyObject(_obj: *mut ::libc::c_void,
                                      obj: *mut ::libc::c_void) -> ();
    pub fn wxJoystickEvent_GetButtonChange(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystickEvent_GetButtonState(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystickEvent_GetJoystick(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystickEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxJoystickEvent_GetZPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystickEvent_IsButton(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystickEvent_IsMove(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxJoystickEvent_IsZMove(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxJoystickEvent_SetButtonChange(_obj: *mut ::libc::c_void,
                                           change: ::libc::c_int) -> ();
    pub fn wxJoystickEvent_SetButtonState(_obj: *mut ::libc::c_void,
                                          state: ::libc::c_int) -> ();
    pub fn wxJoystickEvent_SetJoystick(_obj: *mut ::libc::c_void,
                                       stick: ::libc::c_int) -> ();
    pub fn wxJoystickEvent_SetPosition(_obj: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int)
     -> ();
    pub fn wxJoystickEvent_SetZPosition(_obj: *mut ::libc::c_void,
                                        zPos: ::libc::c_int) -> ();
    pub fn wxKeyEvent_AltDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxKeyEvent_ControlDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxKeyEvent_CopyObject(_obj: *mut ::libc::c_void,
                                 obj: *mut ::libc::c_void) -> ();
    pub fn wxKeyEvent_GetKeyCode(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxKeyEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxKeyEvent_GetX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxKeyEvent_GetY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxKeyEvent_GetModifiers(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxKeyEvent_HasModifiers(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxKeyEvent_MetaDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxKeyEvent_SetKeyCode(_obj: *mut ::libc::c_void,
                                 code: ::libc::c_int) -> ();
    pub fn wxKeyEvent_ShiftDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxLEDNumberCtrl_Create(parent: *mut ::libc::c_void,
                                  id: ::libc::c_int, x: ::libc::c_int,
                                  y: ::libc::c_int, w: ::libc::c_int,
                                  h: ::libc::c_int, style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxLEDNumberCtrl_GetAlignment(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLEDNumberCtrl_GetDrawFaded(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLEDNumberCtrl_GetValue(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLEDNumberCtrl_SetAlignment(_obj: *mut ::libc::c_void,
                                        Alignment: ::libc::c_int,
                                        Redraw: ::libc::c_int) -> ();
    pub fn wxLEDNumberCtrl_SetDrawFaded(_obj: *mut ::libc::c_void,
                                        DrawFaded: ::libc::c_int,
                                        Redraw: ::libc::c_int) -> ();
    pub fn wxLEDNumberCtrl_SetValue(_obj: *mut ::libc::c_void,
                                    Value: *mut ::libc::c_void,
                                    Redraw: ::libc::c_int) -> ();
    pub fn wxLayoutAlgorithm_Create() -> *mut ::libc::c_void;
    pub fn wxLayoutAlgorithm_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLayoutAlgorithm_LayoutFrame(_obj: *mut ::libc::c_void,
                                         frame: *mut ::libc::c_void,
                                         mainWindow: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLayoutAlgorithm_LayoutMDIFrame(_obj: *mut ::libc::c_void,
                                            frame: *mut ::libc::c_void,
                                            x: ::libc::c_int,
                                            y: ::libc::c_int,
                                            w: ::libc::c_int,
                                            h: ::libc::c_int,
                                            _use: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxLayoutAlgorithm_LayoutWindow(_obj: *mut ::libc::c_void,
                                          frame: *mut ::libc::c_void,
                                          mainWindow: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLayoutConstraints_Create() -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_bottom(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_centreX(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_centreY(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_height(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_left(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_right(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_top(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLayoutConstraints_width(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListBox_Append(_obj: *mut ::libc::c_void,
                            item: *mut ::libc::c_void) -> ();
    pub fn wxListBox_AppendData(_obj: *mut ::libc::c_void,
                                item: *mut ::libc::c_void,
                                data: *mut ::libc::c_void) -> ();
    pub fn wxListBox_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxListBox_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                            _lft: ::libc::c_int, _top: ::libc::c_int,
                            _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                            n: ::libc::c_int, str: *mut *mut ::libc::c_char,
                            _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxListBox_Delete(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> ();
    pub fn wxListBox_FindString(_obj: *mut ::libc::c_void,
                                s: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListBox_GetClientData(_obj: *mut ::libc::c_void,
                                   n: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxListBox_GetCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListBox_GetSelection(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListBox_GetSelections(_obj: *mut ::libc::c_void,
                                   aSelections: *mut ::libc::c_int,
                                   allocated: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListBox_GetString(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxListBox_InsertItems(_obj: *mut ::libc::c_void,
                                 items: *mut ::libc::c_void,
                                 pos: ::libc::c_int, count: ::libc::c_int)
     -> ();
    pub fn wxListBox_IsSelected(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListBox_SetClientData(_obj: *mut ::libc::c_void,
                                   n: ::libc::c_int,
                                   clientData: *mut ::libc::c_void) -> ();
    pub fn wxListBox_SetFirstItem(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> ();
    pub fn wxListBox_SetSelection(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                                  select: ::libc::c_int) -> ();
    pub fn wxListBox_SetString(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                               s: *mut ::libc::c_void) -> ();
    pub fn wxListBox_SetStringSelection(_obj: *mut ::libc::c_void,
                                        str: *mut ::libc::c_void,
                                        sel: ::libc::c_int) -> ();
    pub fn wxListCtrl_Arrange(_obj: *mut ::libc::c_void, flag: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListCtrl_ClearAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _lft: ::libc::c_int, _top: ::libc::c_int,
                             _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                             _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxListCtrl_DeleteAllColumns(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_DeleteAllItems(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_DeleteColumn(_obj: *mut ::libc::c_void,
                                   col: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_DeleteItem(_obj: *mut ::libc::c_void,
                                 item: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_EditLabel(_obj: *mut ::libc::c_void,
                                item: ::libc::c_int) -> ();
    pub fn wxListCtrl_EndEditLabel(_obj: *mut ::libc::c_void,
                                   cancel: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_EnsureVisible(_obj: *mut ::libc::c_void,
                                    item: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_FindItem(_obj: *mut ::libc::c_void,
                               start: ::libc::c_int, str: *mut ::libc::c_void,
                               partial: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_FindItemByData(_obj: *mut ::libc::c_void,
                                     start: ::libc::c_int,
                                     data: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_FindItemByPosition(_obj: *mut ::libc::c_void,
                                         start: ::libc::c_int,
                                         x: ::libc::c_int, y: ::libc::c_int,
                                         direction: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListCtrl_GetColumn(_obj: *mut ::libc::c_void, col: ::libc::c_int,
                                item: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_GetColumnCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_GetColumnWidth(_obj: *mut ::libc::c_void,
                                     col: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_GetCountPerPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_GetEditControl(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetImageList(_obj: *mut ::libc::c_void,
                                   which: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetItem(_obj: *mut ::libc::c_void,
                              info: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_GetItemCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_GetItemData(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_GetItemFont(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetItemPosition(_obj: *mut ::libc::c_void,
                                      item: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetItemRect(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_int, code: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetItemSpacing(_obj: *mut ::libc::c_void,
                                     isSmall: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetItemState(_obj: *mut ::libc::c_void,
                                   item: ::libc::c_int,
                                   stateMask: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_GetItemText(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxListCtrl_GetNextItem(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_int,
                                  geometry: ::libc::c_int,
                                  state: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_GetSelectedItemCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_GetTextColour(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_GetTopItem(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_HitTest(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, flags: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_InsertColumn(_obj: *mut ::libc::c_void,
                                   col: ::libc::c_int,
                                   heading: *mut ::libc::c_void,
                                   format: ::libc::c_int,
                                   width: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_InsertColumnFromInfo(_obj: *mut ::libc::c_void,
                                           col: ::libc::c_int,
                                           info: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_InsertItem(_obj: *mut ::libc::c_void,
                                 info: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_InsertItemWithData(_obj: *mut ::libc::c_void,
                                         index: ::libc::c_int,
                                         label: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_InsertItemWithImage(_obj: *mut ::libc::c_void,
                                          index: ::libc::c_int,
                                          imageIndex: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListCtrl_InsertItemWithLabel(_obj: *mut ::libc::c_void,
                                          index: ::libc::c_int,
                                          label: *mut ::libc::c_void,
                                          imageIndex: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListCtrl_IsVirtual(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_RefreshItem(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_long) -> ();
    pub fn wxListCtrl_ScrollList(_obj: *mut ::libc::c_void, dx: ::libc::c_int,
                                 dy: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                          col: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_SetColumn(_obj: *mut ::libc::c_void, col: ::libc::c_int,
                                item: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_SetColumnWidth(_obj: *mut ::libc::c_void,
                                     col: ::libc::c_int, width: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListCtrl_SetForegroundColour(_obj: *mut ::libc::c_void,
                                          col: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_SetImageList(_obj: *mut ::libc::c_void,
                                   imageList: *mut ::libc::c_void,
                                   which: ::libc::c_int) -> ();
    pub fn wxListCtrl_SetItem(_obj: *mut ::libc::c_void, index: ::libc::c_int,
                              col: ::libc::c_int, label: *mut ::libc::c_void,
                              imageId: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_SetItemData(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_int, data: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxListCtrl_SetItemFromInfo(_obj: *mut ::libc::c_void,
                                      info: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_SetItemImage(_obj: *mut ::libc::c_void,
                                   item: ::libc::c_int, image: ::libc::c_int,
                                   selImage: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_SetItemPosition(_obj: *mut ::libc::c_void,
                                      item: ::libc::c_int, x: ::libc::c_int,
                                      y: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_SetItemState(_obj: *mut ::libc::c_void,
                                   item: ::libc::c_int, state: ::libc::c_int,
                                   stateMask: ::libc::c_int) -> ::libc::c_int;
    pub fn wxListCtrl_SetItemText(_obj: *mut ::libc::c_void,
                                  item: ::libc::c_int,
                                  str: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_SetSingleStyle(_obj: *mut ::libc::c_void,
                                     style: ::libc::c_int, add: ::libc::c_int)
     -> ();
    pub fn wxListCtrl_SetTextColour(_obj: *mut ::libc::c_void,
                                    col: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_SetWindowStyleFlag(_obj: *mut ::libc::c_void,
                                         style: ::libc::c_int) -> ();
    pub fn wxListCtrl_SortItems(_obj: *mut ::libc::c_void,
                                _fn: *mut ::libc::c_void,
                                eif_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListCtrl_UpdateStyle(_obj: *mut ::libc::c_void) -> ();
    pub fn wxListEvent_Cancelled(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetCode(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetColumn(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetData(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetImage(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetIndex(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetItem(_obj: *mut ::libc::c_void,
                               _ref: *mut ::libc::c_void) -> ();
    pub fn wxListEvent_GetLabel(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListEvent_GetMask(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListEvent_GetPoint(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListEvent_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListItem_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxListItem_ClearAttributes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxListItem_Create() -> *mut ::libc::c_void;
    pub fn wxListItem_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxListItem_GetAlign(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetAttributes(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListItem_GetBackgroundColour(_obj: *mut ::libc::c_void,
                                          _ref: *mut ::libc::c_void) -> ();
    pub fn wxListItem_GetColumn(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetData(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetFont(_obj: *mut ::libc::c_void,
                              _ref: *mut ::libc::c_void) -> ();
    pub fn wxListItem_GetId(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetImage(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetMask(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetState(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListItem_GetTextColour(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxListItem_GetWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListItem_HasAttributes(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListItem_SetAlign(_obj: *mut ::libc::c_void,
                               align: ::libc::c_int) -> ();
    pub fn wxListItem_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                          colBack: *mut ::libc::c_void) -> ();
    pub fn wxListItem_SetColumn(_obj: *mut ::libc::c_void, col: ::libc::c_int)
     -> ();
    pub fn wxListItem_SetData(_obj: *mut ::libc::c_void, data: ::libc::c_int)
     -> ();
    pub fn wxListItem_SetDataPointer(_obj: *mut ::libc::c_void,
                                     data: *mut ::libc::c_void) -> ();
    pub fn wxListItem_SetFont(_obj: *mut ::libc::c_void,
                              font: *mut ::libc::c_void) -> ();
    pub fn wxListItem_SetId(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ();
    pub fn wxListItem_SetImage(_obj: *mut ::libc::c_void,
                               image: ::libc::c_int) -> ();
    pub fn wxListItem_SetMask(_obj: *mut ::libc::c_void, mask: ::libc::c_int)
     -> ();
    pub fn wxListItem_SetState(_obj: *mut ::libc::c_void,
                               state: ::libc::c_int) -> ();
    pub fn wxListItem_SetStateMask(_obj: *mut ::libc::c_void,
                                   stateMask: ::libc::c_int) -> ();
    pub fn wxListItem_SetText(_obj: *mut ::libc::c_void,
                              text: *mut ::libc::c_void) -> ();
    pub fn wxListItem_SetTextColour(_obj: *mut ::libc::c_void,
                                    colText: *mut ::libc::c_void) -> ();
    pub fn wxListItem_SetWidth(_obj: *mut ::libc::c_void,
                               width: ::libc::c_int) -> ();
    pub fn wxLocale_AddCatalog(_obj: *mut ::libc::c_void,
                               szDomain: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLocale_AddCatalogLookupPathPrefix(_obj: *mut ::libc::c_void,
                                               prefix: *mut ::libc::c_void)
     -> ();
    pub fn wxLocale_Create(_name: ::libc::c_int, _flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxLocale_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLocale_GetLocale(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLocale_GetName(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxLocale_GetString(_obj: *mut ::libc::c_void,
                              szOrigString: *mut ::libc::c_void,
                              szDomain: *mut ::libc::c_void)
     -> *mut ::libc::c_char;
    pub fn wxLocale_IsLoaded(_obj: *mut ::libc::c_void,
                             szDomain: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxLocale_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxLogChain_Create(logger: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLogChain_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLogChain_GetOldLog(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLogChain_IsPassingMessages(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLogChain_PassMessages(_obj: *mut ::libc::c_void,
                                   bDoPass: ::libc::c_int) -> ();
    pub fn wxLogChain_SetLog(_obj: *mut ::libc::c_void,
                             logger: *mut ::libc::c_void) -> ();
    pub fn wxMDIChildFrame_Activate(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMDIChildFrame_Create(_prt: *mut ::libc::c_void,
                                  _id: ::libc::c_int,
                                  _txt: *mut ::libc::c_void,
                                  _lft: ::libc::c_int, _top: ::libc::c_int,
                                  _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                  _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxMDIParentFrame_ActivateNext(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMDIParentFrame_ActivatePrevious(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMDIParentFrame_ArrangeIcons(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMDIParentFrame_Cascade(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMDIParentFrame_Create(_prt: *mut ::libc::c_void,
                                   _id: ::libc::c_int,
                                   _txt: *mut ::libc::c_void,
                                   _lft: ::libc::c_int, _top: ::libc::c_int,
                                   _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                   _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMDIParentFrame_GetActiveChild(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMDIParentFrame_GetClientWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMDIParentFrame_GetWindowMenu(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMDIParentFrame_OnCreateClient(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMDIParentFrame_SetWindowMenu(_obj: *mut ::libc::c_void,
                                          menu: *mut ::libc::c_void) -> ();
    pub fn wxMDIParentFrame_Tile(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMask_Create(bitmap: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxMask_CreateColoured(bitmap: *mut ::libc::c_void,
                                 colour: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMemoryDC_Create() -> *mut ::libc::c_void;
    pub fn wxMemoryDC_CreateCompatible(dc: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMemoryDC_CreateWithBitmap(bitmap: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMemoryDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMemoryDC_SelectObject(_obj: *mut ::libc::c_void,
                                   bitmap: *mut ::libc::c_void) -> ();
    pub fn wxMenu_Append(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                         text: *mut ::libc::c_void, help: *mut ::libc::c_void,
                         isCheckable: ::libc::c_int) -> ();
    pub fn wxMenu_AppendItem(_obj: *mut ::libc::c_void,
                             _itm: *mut ::libc::c_void) -> ();
    pub fn wxMenu_AppendSeparator(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMenu_AppendSub(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                            text: *mut ::libc::c_void,
                            submenu: *mut ::libc::c_void,
                            help: *mut ::libc::c_void) -> ();
    pub fn wxMenu_Break(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMenu_Check(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                        check: ::libc::c_int) -> ();
    pub fn wxMenu_Create(title: *mut ::libc::c_void, style: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxMenu_DeleteById(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ();
    pub fn wxMenu_DeleteByItem(_obj: *mut ::libc::c_void,
                               _itm: *mut ::libc::c_void) -> ();
    pub fn wxMenu_DeletePointer(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMenu_DestroyById(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ();
    pub fn wxMenu_DestroyByItem(_obj: *mut ::libc::c_void,
                                _itm: *mut ::libc::c_void) -> ();
    pub fn wxMenu_Enable(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                         enable: ::libc::c_int) -> ();
    pub fn wxMenu_FindItem(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenu_FindItemByLabel(_obj: *mut ::libc::c_void,
                                  itemString: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMenu_GetClientData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenu_GetHelpString(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenu_GetInvokingWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenu_GetLabel(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenu_GetMenuItemCount(_obj: *mut ::libc::c_void) -> size_t;
    pub fn wxMenu_GetMenuItems(_obj: *mut ::libc::c_void,
                               _lst: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenu_GetParent(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxMenu_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenu_GetTitle(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxMenu_Insert(_obj: *mut ::libc::c_void, pos: size_t,
                         id: ::libc::c_int, text: *mut ::libc::c_void,
                         help: *mut ::libc::c_void,
                         isCheckable: ::libc::c_int) -> ();
    pub fn wxMenu_InsertItem(_obj: *mut ::libc::c_void, pos: size_t,
                             _itm: *mut ::libc::c_void) -> ();
    pub fn wxMenu_InsertSub(_obj: *mut ::libc::c_void, pos: size_t,
                            id: ::libc::c_int, text: *mut ::libc::c_void,
                            submenu: *mut ::libc::c_void,
                            help: *mut ::libc::c_void) -> ();
    pub fn wxMenu_IsAttached(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenu_IsChecked(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMenu_IsEnabled(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMenu_Prepend(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                          text: *mut ::libc::c_void,
                          help: *mut ::libc::c_void,
                          isCheckable: ::libc::c_int) -> ();
    pub fn wxMenu_PrependItem(_obj: *mut ::libc::c_void,
                              _itm: *mut ::libc::c_void) -> ();
    pub fn wxMenu_PrependSub(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                             text: *mut ::libc::c_void,
                             submenu: *mut ::libc::c_void,
                             help: *mut ::libc::c_void) -> ();
    pub fn wxMenu_RemoveById(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                             _itm: *mut ::libc::c_void) -> ();
    pub fn wxMenu_RemoveByItem(_obj: *mut ::libc::c_void,
                               item: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetClientData(_obj: *mut ::libc::c_void,
                                clientData: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetEventHandler(_obj: *mut ::libc::c_void,
                                  handler: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetHelpString(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                                helpString: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetInvokingWindow(_obj: *mut ::libc::c_void,
                                    win: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetLabel(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                           label: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetParent(_obj: *mut ::libc::c_void,
                            parent: *mut ::libc::c_void) -> ();
    pub fn wxMenu_SetTitle(_obj: *mut ::libc::c_void,
                           title: *mut ::libc::c_void) -> ();
    pub fn wxMenu_UpdateUI(_obj: *mut ::libc::c_void,
                           source: *mut ::libc::c_void) -> ();
    pub fn wxMenuBar_Append(_obj: *mut ::libc::c_void,
                            menu: *mut ::libc::c_void,
                            title: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuBar_Check(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                           check: ::libc::c_int) -> ();
    pub fn wxMenuBar_Create(_style: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxMenuBar_DeletePointer(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMenuBar_Enable(_obj: *mut ::libc::c_void, enable: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMenuBar_EnableItem(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                                enable: ::libc::c_int) -> ();
    pub fn wxMenuBar_EnableTop(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                               enable: ::libc::c_int) -> ();
    pub fn wxMenuBar_FindItem(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenuBar_FindMenu(_obj: *mut ::libc::c_void,
                              title: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuBar_FindMenuItem(_obj: *mut ::libc::c_void,
                                  menuString: *mut ::libc::c_void,
                                  itemString: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMenuBar_GetHelpString(_obj: *mut ::libc::c_void,
                                   id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxMenuBar_GetLabel(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenuBar_GetLabelTop(_obj: *mut ::libc::c_void,
                                 pos: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxMenuBar_GetMenu(_obj: *mut ::libc::c_void, pos: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenuBar_GetMenuCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuBar_Insert(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                            menu: *mut ::libc::c_void,
                            title: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuBar_IsChecked(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMenuBar_IsEnabled(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMenuBar_Remove(_obj: *mut ::libc::c_void, pos: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMenuBar_Replace(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                             menu: *mut ::libc::c_void,
                             title: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuBar_SetHelpString(_obj: *mut ::libc::c_void,
                                   id: ::libc::c_int,
                                   helpString: *mut ::libc::c_void) -> ();
    pub fn wxMenuBar_SetItemLabel(_obj: *mut ::libc::c_void,
                                  id: ::libc::c_int,
                                  label: *mut ::libc::c_void) -> ();
    pub fn wxMenuBar_SetLabel(_obj: *mut ::libc::c_void,
                              s: *mut ::libc::c_void) -> ();
    pub fn wxMenuBar_SetLabelTop(_obj: *mut ::libc::c_void,
                                 pos: ::libc::c_int,
                                 label: *mut ::libc::c_void) -> ();
    pub fn wxMenuEvent_CopyObject(_obj: *mut ::libc::c_void,
                                  obj: *mut ::libc::c_void) -> ();
    pub fn wxMenuEvent_GetMenuId(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_Check(_obj: *mut ::libc::c_void, check: ::libc::c_int)
     -> ();
    pub fn wxMenuItem_Create() -> *mut ::libc::c_void;
    pub fn wxMenuItem_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMenuItem_Enable(_obj: *mut ::libc::c_void, enable: ::libc::c_int)
     -> ();
    pub fn wxMenuItem_GetHelp(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuItem_GetId(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_GetLabel(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuItem_GetLabelFromText(text: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuItem_GetMenu(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuItem_GetSubMenu(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuItem_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuItem_IsCheckable(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_IsChecked(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_IsEnabled(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_IsSeparator(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_IsSubMenu(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMenuItem_SetCheckable(_obj: *mut ::libc::c_void,
                                   checkable: ::libc::c_int) -> ();
    pub fn wxMenuItem_SetHelp(_obj: *mut ::libc::c_void,
                              str: *mut ::libc::c_void) -> ();
    pub fn wxMenuItem_SetId(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ();
    pub fn wxMenuItem_SetSubMenu(_obj: *mut ::libc::c_void,
                                 menu: *mut ::libc::c_void) -> ();
    pub fn wxMenuItem_SetText(_obj: *mut ::libc::c_void,
                              str: *mut ::libc::c_void) -> ();
    pub fn wxMessageDialog_Create(_prt: *mut ::libc::c_void,
                                  _msg: *mut ::libc::c_void,
                                  _cap: *mut ::libc::c_void,
                                  _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxMessageDialog_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMessageDialog_ShowModal(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMetafile_Create(_file: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMetafile_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMetafile_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMetafile_Play(_obj: *mut ::libc::c_void,
                           _dc: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMetafile_SetClipboard(_obj: *mut ::libc::c_void,
                                   width: ::libc::c_int,
                                   height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxMetafileDC_Close(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMetafileDC_Create(_file: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMetafileDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMimeTypesManager_AddFallbacks(_obj: *mut ::libc::c_void,
                                           _types: *mut ::libc::c_void) -> ();
    pub fn wxMimeTypesManager_Create() -> *mut ::libc::c_void;
    pub fn wxMimeTypesManager_EnumAllFileTypes(_obj: *mut ::libc::c_void,
                                               _lst: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMimeTypesManager_GetFileTypeFromExtension(_obj:
                                                           *mut ::libc::c_void,
                                                       _ext:
                                                           *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMimeTypesManager_GetFileTypeFromMimeType(_obj:
                                                          *mut ::libc::c_void,
                                                      _name:
                                                          *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMimeTypesManager_IsOfType(_obj: *mut ::libc::c_void,
                                       _type: *mut ::libc::c_void,
                                       _wildcard: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMiniFrame_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                              _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                              _top: ::libc::c_int, _wdt: ::libc::c_int,
                              _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMirrorDC_Create(dc: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxMirrorDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMouseEvent_AltDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_Button(_obj: *mut ::libc::c_void, but: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMouseEvent_ButtonDClick(_obj: *mut ::libc::c_void,
                                     but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxMouseEvent_ButtonDown(_obj: *mut ::libc::c_void,
                                   but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxMouseEvent_ButtonIsDown(_obj: *mut ::libc::c_void,
                                     but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxMouseEvent_ButtonUp(_obj: *mut ::libc::c_void,
                                 but: ::libc::c_int) -> ::libc::c_int;
    pub fn wxMouseEvent_ControlDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_CopyObject(_obj: *mut ::libc::c_void,
                                   object_dest: *mut ::libc::c_void) -> ();
    pub fn wxMouseEvent_Dragging(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_Entering(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_GetLogicalPosition(_obj: *mut ::libc::c_void,
                                           dc: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMouseEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMouseEvent_GetX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_GetY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_IsButton(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_Leaving(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_LeftDClick(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_LeftDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_LeftIsDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_LeftUp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_MetaDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_MiddleDClick(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_MiddleDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_MiddleIsDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_MiddleUp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_Moving(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_RightDClick(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_RightDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_RightIsDown(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_RightUp(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMouseEvent_ShiftDown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMoveEvent_CopyObject(_obj: *mut ::libc::c_void,
                                  obj: *mut ::libc::c_void) -> ();
    pub fn wxMoveEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMultiCellCanvas_Add(_obj: *mut ::libc::c_void,
                                 win: *mut ::libc::c_void, row: ::libc::c_int,
                                 col: ::libc::c_int) -> ();
    pub fn wxMultiCellCanvas_CalculateConstraints(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxMultiCellCanvas_Create(parent: *mut ::libc::c_void,
                                    numRows: ::libc::c_int,
                                    numCols: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMultiCellCanvas_MaxCols(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellCanvas_MaxRows(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellCanvas_SetMinCellSize(_obj: *mut ::libc::c_void,
                                            w: ::libc::c_int,
                                            h: ::libc::c_int) -> ();
    pub fn wxMultiCellItemHandle_Create(row: ::libc::c_int,
                                        column: ::libc::c_int,
                                        height: ::libc::c_int,
                                        width: ::libc::c_int,
                                        sx: ::libc::c_int, sy: ::libc::c_int,
                                        style: ::libc::c_int,
                                        wx: ::libc::c_int, wy: ::libc::c_int,
                                        align: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMultiCellItemHandle_CreateWithSize(_obj: *mut ::libc::c_void,
                                                row: ::libc::c_int,
                                                column: ::libc::c_int,
                                                sx: ::libc::c_int,
                                                sy: ::libc::c_int,
                                                style: ::libc::c_int,
                                                wx: ::libc::c_int,
                                                wy: ::libc::c_int,
                                                align: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMultiCellItemHandle_CreateWithStyle(_obj: *mut ::libc::c_void,
                                                 row: ::libc::c_int,
                                                 column: ::libc::c_int,
                                                 style: ::libc::c_int,
                                                 wx: ::libc::c_int,
                                                 wy: ::libc::c_int,
                                                 align: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMultiCellItemHandle_GetAlignment(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellItemHandle_GetColumn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellItemHandle_GetHeight(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellItemHandle_GetLocalSize(_obj: *mut ::libc::c_void,
                                              _w: *mut ::libc::c_void,
                                              _h: *mut ::libc::c_void) -> ();
    pub fn wxMultiCellItemHandle_GetRow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellItemHandle_GetStyle(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellItemHandle_GetWeight(_obj: *mut ::libc::c_void,
                                           _w: *mut ::libc::c_void,
                                           _h: *mut ::libc::c_void) -> ();
    pub fn wxMultiCellItemHandle_GetWidth(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellSizer_CalcMin(_obj: *mut ::libc::c_void,
                                    _w: *mut ::libc::c_void,
                                    _h: *mut ::libc::c_void) -> ();
    pub fn wxMultiCellSizer_Create(rows: ::libc::c_int, cols: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxMultiCellSizer_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMultiCellSizer_EnableGridLines(_obj: *mut ::libc::c_void,
                                            win: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellSizer_RecalcSizes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMultiCellSizer_SetColumnWidth(_obj: *mut ::libc::c_void,
                                           column: ::libc::c_int,
                                           colSize: ::libc::c_int,
                                           expandable: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMultiCellSizer_SetDefaultCellSize(_obj: *mut ::libc::c_void,
                                               w: ::libc::c_int,
                                               h: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMultiCellSizer_SetGridPen(_obj: *mut ::libc::c_void,
                                       pen: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMultiCellSizer_SetRowHeight(_obj: *mut ::libc::c_void,
                                         row: ::libc::c_int,
                                         rowSize: ::libc::c_int,
                                         expandable: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMutex_Create() -> *mut ::libc::c_void;
    pub fn wxMutex_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMutex_IsLocked(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMutex_Lock(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMutex_TryLock(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMutex_Unlock(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxNavigationKeyEvent_GetCurrentFocus(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxNavigationKeyEvent_GetDirection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNavigationKeyEvent_IsWindowChange(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNavigationKeyEvent_SetCurrentFocus(_obj: *mut ::libc::c_void,
                                                win: *mut ::libc::c_void)
     -> ();
    pub fn wxNavigationKeyEvent_SetDirection(_obj: *mut ::libc::c_void,
                                             bForward: ::libc::c_int) -> ();
    pub fn wxNavigationKeyEvent_SetWindowChange(_obj: *mut ::libc::c_void,
                                                bIs: ::libc::c_int) -> ();
    pub fn wxNavigationKeyEvent_ShouldPropagate(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNewBitmapButton_Create(labelBitmap: *mut ::libc::c_void,
                                    labelText: *mut ::libc::c_void,
                                    alignText: ::libc::c_int,
                                    isFlat: ::libc::c_int,
                                    firedEventType: ::libc::c_int,
                                    marginX: ::libc::c_int,
                                    marginY: ::libc::c_int,
                                    textToLabelGap: ::libc::c_int,
                                    isSticky: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxNewBitmapButton_CreateFromFile(bitmapFileName:
                                                *mut ::libc::c_void,
                                            bitmapFileType: ::libc::c_int,
                                            labelText: *mut ::libc::c_void,
                                            alignText: ::libc::c_int,
                                            isFlat: ::libc::c_int,
                                            firedEventType: ::libc::c_int,
                                            marginX: ::libc::c_int,
                                            marginY: ::libc::c_int,
                                            textToLabelGap: ::libc::c_int,
                                            isSticky: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxNewBitmapButton_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxNewBitmapButton_DrawDecorations(_obj: *mut ::libc::c_void,
                                             dc: *mut ::libc::c_void) -> ();
    pub fn wxNewBitmapButton_DrawLabel(_obj: *mut ::libc::c_void,
                                       dc: *mut ::libc::c_void) -> ();
    pub fn wxNewBitmapButton_Enable(_obj: *mut ::libc::c_void,
                                    enable: ::libc::c_int) -> ::libc::c_int;
    pub fn wxNewBitmapButton_Realize(_obj: *mut ::libc::c_void,
                                     _prt: *mut ::libc::c_void,
                                     _id: ::libc::c_int, _x: ::libc::c_int,
                                     _y: ::libc::c_int, _w: ::libc::c_int,
                                     _h: ::libc::c_int) -> ();
    pub fn wxNewBitmapButton_RenderAllLabelImages(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxNewBitmapButton_RenderLabelImage(_obj: *mut ::libc::c_void,
                                              destBmp: *mut ::libc::c_void,
                                              srcBmp: *mut ::libc::c_void,
                                              isEnabled: ::libc::c_int,
                                              isPressed: ::libc::c_int) -> ();
    pub fn wxNewBitmapButton_RenderLabelImages(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxNewBitmapButton_Reshape(_obj: *mut ::libc::c_void) -> ();
    pub fn wxNewBitmapButton_SetAlignments(_obj: *mut ::libc::c_void,
                                           alignText: ::libc::c_int,
                                           marginX: ::libc::c_int,
                                           marginY: ::libc::c_int,
                                           textToLabelGap: ::libc::c_int)
     -> ();
    pub fn wxNewBitmapButton_SetLabel(_obj: *mut ::libc::c_void,
                                      labelBitmap: *mut ::libc::c_void,
                                      labelText: *mut ::libc::c_void) -> ();
    pub fn wxNotebook_AddPage(_obj: *mut ::libc::c_void,
                              pPage: *mut ::libc::c_void,
                              strText: *mut ::libc::c_void,
                              bSelect: ::libc::c_int, imageId: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxNotebook_AdvanceSelection(_obj: *mut ::libc::c_void,
                                       bForward: ::libc::c_int) -> ();
    pub fn wxNotebook_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _lft: ::libc::c_int, _top: ::libc::c_int,
                             _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                             _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxNotebook_DeleteAllPages(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNotebook_DeletePage(_obj: *mut ::libc::c_void,
                                 nPage: ::libc::c_int) -> ::libc::c_int;
    pub fn wxNotebook_GetImageList(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxNotebook_GetPage(_obj: *mut ::libc::c_void, nPage: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxNotebook_GetPageCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNotebook_GetPageImage(_obj: *mut ::libc::c_void,
                                   nPage: ::libc::c_int) -> ::libc::c_int;
    pub fn wxNotebook_GetPageText(_obj: *mut ::libc::c_void,
                                  nPage: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxNotebook_GetRowCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxNotebook_GetSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNotebook_HitTest(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, flags: *mut ::libc::c_long)
     -> ::libc::c_int;
    pub fn wxNotebook_InsertPage(_obj: *mut ::libc::c_void,
                                 nPage: ::libc::c_int,
                                 pPage: *mut ::libc::c_void,
                                 strText: *mut ::libc::c_void,
                                 bSelect: ::libc::c_int,
                                 imageId: ::libc::c_int) -> ::libc::c_int;
    pub fn wxNotebook_RemovePage(_obj: *mut ::libc::c_void,
                                 nPage: ::libc::c_int) -> ::libc::c_int;
    pub fn wxNotebook_SetImageList(_obj: *mut ::libc::c_void,
                                   imageList: *mut ::libc::c_void) -> ();
    pub fn wxNotebook_SetPadding(_obj: *mut ::libc::c_void, _w: ::libc::c_int,
                                 _h: ::libc::c_int) -> ();
    pub fn wxNotebook_SetPageImage(_obj: *mut ::libc::c_void,
                                   nPage: ::libc::c_int,
                                   nImage: ::libc::c_int) -> ::libc::c_int;
    pub fn wxNotebook_SetPageSize(_obj: *mut ::libc::c_void,
                                  _w: ::libc::c_int, _h: ::libc::c_int) -> ();
    pub fn wxNotebook_SetPageText(_obj: *mut ::libc::c_void,
                                  nPage: ::libc::c_int,
                                  strText: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNotebook_SetSelection(_obj: *mut ::libc::c_void,
                                   nPage: ::libc::c_int) -> ::libc::c_int;
    pub fn expNB_TOP() -> ::libc::c_int;
    pub fn expNB_BOTTOM() -> ::libc::c_int;
    pub fn expNB_LEFT() -> ::libc::c_int;
    pub fn expNB_RIGHT() -> ::libc::c_int;
    pub fn expBK_HITTEST_NOWHERE() -> ::libc::c_int;
    pub fn expBK_HITTEST_ONICON() -> ::libc::c_int;
    pub fn expBK_HITTEST_ONLABEL() -> ::libc::c_int;
    pub fn expBK_HITTEST_ONITEM() -> ::libc::c_int;
    pub fn expBK_HITTEST_ONPAGE() -> ::libc::c_int;
    pub fn wxNotifyEvent_Allow(_obj: *mut ::libc::c_void) -> ();
    pub fn wxNotifyEvent_CopyObject(_obj: *mut ::libc::c_void,
                                    object_dest: *mut ::libc::c_void) -> ();
    pub fn wxNotifyEvent_IsAllowed(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNotifyEvent_Veto(_obj: *mut ::libc::c_void) -> ();
    pub fn wxOutputStream_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxOutputStream_LastWrite(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxOutputStream_PutC(_obj: *mut ::libc::c_void, c: ::libc::c_char)
     -> ();
    pub fn wxOutputStream_Seek(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                               mode: ::libc::c_int) -> ::libc::c_int;
    pub fn wxOutputStream_Sync(_obj: *mut ::libc::c_void) -> ();
    pub fn wxOutputStream_Tell(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxOutputStream_Write(_obj: *mut ::libc::c_void,
                                buffer: *mut ::libc::c_void,
                                size: ::libc::c_int) -> ();
    pub fn wxPageSetupDialog_Create(parent: *mut ::libc::c_void,
                                    data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialog_GetPageSetupData(_obj: *mut ::libc::c_void,
                                              _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxPageSetupDialogData_Assign(_obj: *mut ::libc::c_void,
                                        data: *mut ::libc::c_void) -> ();
    pub fn wxPageSetupDialogData_AssignData(_obj: *mut ::libc::c_void,
                                            printData: *mut ::libc::c_void)
     -> ();
    pub fn wxPageSetupDialogData_CalculateIdFromPaperSize(_obj:
                                                              *mut ::libc::c_void)
     -> ();
    pub fn wxPageSetupDialogData_CalculatePaperSizeFromId(_obj:
                                                              *mut ::libc::c_void)
     -> ();
    pub fn wxPageSetupDialogData_Create() -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_CreateFromData(printData:
                                                    *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPageSetupDialogData_EnableHelp(_obj: *mut ::libc::c_void,
                                            flag: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_EnableMargins(_obj: *mut ::libc::c_void,
                                               flag: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_EnableOrientation(_obj: *mut ::libc::c_void,
                                                   flag: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_EnablePaper(_obj: *mut ::libc::c_void,
                                             flag: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_EnablePrinter(_obj: *mut ::libc::c_void,
                                               flag: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_GetDefaultInfo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetDefaultMinMargins(_obj:
                                                          *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetEnableHelp(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetEnableMargins(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetEnableOrientation(_obj:
                                                          *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetEnablePaper(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetEnablePrinter(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetMarginBottomRight(_obj:
                                                          *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_GetMarginTopLeft(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_GetMinMarginBottomRight(_obj:
                                                             *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_GetMinMarginTopLeft(_obj:
                                                         *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_GetPaperId(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPageSetupDialogData_GetPaperSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPageSetupDialogData_GetPrintData(_obj: *mut ::libc::c_void,
                                              _ref: *mut ::libc::c_void)
     -> ();
    pub fn wxPageSetupDialogData_SetDefaultInfo(_obj: *mut ::libc::c_void,
                                                flag: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_SetDefaultMinMargins(_obj:
                                                          *mut ::libc::c_void,
                                                      flag: ::libc::c_int)
     -> ();
    pub fn wxPageSetupDialogData_SetMarginBottomRight(_obj:
                                                          *mut ::libc::c_void,
                                                      x: ::libc::c_int,
                                                      y: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_SetMarginTopLeft(_obj: *mut ::libc::c_void,
                                                  x: ::libc::c_int,
                                                  y: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_SetMinMarginBottomRight(_obj:
                                                             *mut ::libc::c_void,
                                                         x: ::libc::c_int,
                                                         y: ::libc::c_int)
     -> ();
    pub fn wxPageSetupDialogData_SetMinMarginTopLeft(_obj:
                                                         *mut ::libc::c_void,
                                                     x: ::libc::c_int,
                                                     y: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_SetPaperId(_obj: *mut ::libc::c_void,
                                            id: *mut ::libc::c_void) -> ();
    pub fn wxPageSetupDialogData_SetPaperSize(_obj: *mut ::libc::c_void,
                                              w: ::libc::c_int,
                                              h: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_SetPaperSizeId(_obj: *mut ::libc::c_void,
                                                id: ::libc::c_int) -> ();
    pub fn wxPageSetupDialogData_SetPrintData(_obj: *mut ::libc::c_void,
                                              printData: *mut ::libc::c_void)
     -> ();
    pub fn wxPaintDC_Create(win: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxPaintDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPalette_Assign(_obj: *mut ::libc::c_void,
                            palette: *mut ::libc::c_void) -> ();
    pub fn wxPalette_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxPalette_CreateRGB(n: ::libc::c_int, red: *mut ::libc::c_void,
                               green: *mut ::libc::c_void,
                               blue: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPalette_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPalette_GetPixel(_obj: *mut ::libc::c_void, red: uint8_t,
                              green: uint8_t, blue: uint8_t) -> ::libc::c_int;
    pub fn wxPalette_GetRGB(_obj: *mut ::libc::c_void, pixel: ::libc::c_int,
                            red: *mut ::libc::c_void,
                            green: *mut ::libc::c_void,
                            blue: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPalette_IsEqual(_obj: *mut ::libc::c_void,
                             palette: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPalette_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPaletteChangedEvent_CopyObject(_obj: *mut ::libc::c_void,
                                            obj: *mut ::libc::c_void) -> ();
    pub fn wxPaletteChangedEvent_GetChangedWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPaletteChangedEvent_SetChangedWindow(_obj: *mut ::libc::c_void,
                                                  win: *mut ::libc::c_void)
     -> ();
    pub fn wxPanel_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                          _lft: ::libc::c_int, _top: ::libc::c_int,
                          _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                          _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxPanel_InitDialog(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPanel_SetFocus(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPen_Assign(_obj: *mut ::libc::c_void, pen: *mut ::libc::c_void)
     -> ();
    pub fn wxPen_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxPen_CreateFromBitmap(stipple: *mut ::libc::c_void,
                                  width: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPen_CreateFromColour(col: *mut ::libc::c_void,
                                  width: ::libc::c_int, style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPen_CreateFromStock(id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxPen_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPen_GetCap(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_GetColour(_obj: *mut ::libc::c_void,
                           _ref: *mut ::libc::c_void) -> ();
    pub fn wxPen_GetDashes(_obj: *mut ::libc::c_void,
                           ptr: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_GetJoin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_GetStipple(_obj: *mut ::libc::c_void,
                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxPen_GetStyle(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_GetWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_IsEqual(_obj: *mut ::libc::c_void, pen: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPen_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_SetCap(_obj: *mut ::libc::c_void, cap: ::libc::c_int) -> ();
    pub fn wxPen_SetColour(_obj: *mut ::libc::c_void,
                           col: *mut ::libc::c_void) -> ();
    pub fn wxPen_SetColourSingle(_obj: *mut ::libc::c_void, r: ::libc::c_char,
                                 g: ::libc::c_char, b: ::libc::c_char) -> ();
    pub fn wxPen_SetDashes(_obj: *mut ::libc::c_void,
                           nb_dashes: ::libc::c_int,
                           dash: *mut ::libc::c_void) -> ();
    pub fn wxPen_SetJoin(_obj: *mut ::libc::c_void, join: ::libc::c_int)
     -> ();
    pub fn wxPen_SetStipple(_obj: *mut ::libc::c_void,
                            stipple: *mut ::libc::c_void) -> ();
    pub fn wxPen_SetStyle(_obj: *mut ::libc::c_void, style: ::libc::c_int)
     -> ();
    pub fn wxPen_SetWidth(_obj: *mut ::libc::c_void, width: ::libc::c_int)
     -> ();
    pub fn wxPlotEvent_GetCurve(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPlotEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotEvent_GetZoom(_obj: *mut ::libc::c_void) -> ::libc::c_double;
    pub fn wxPlotEvent_SetPosition(_obj: *mut ::libc::c_void,
                                   pos: ::libc::c_int) -> ();
    pub fn wxPlotEvent_SetZoom(_obj: *mut ::libc::c_void,
                               zoom: ::libc::c_double) -> ();
    pub fn wxPlotOnOffCurve_Add(_obj: *mut ::libc::c_void, on: ::libc::c_int,
                                off: ::libc::c_int,
                                clientData: *mut ::libc::c_void) -> ();
    pub fn wxPlotOnOffCurve_Create(offsetY: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPlotOnOffCurve_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPlotOnOffCurve_DrawOffLine(_obj: *mut ::libc::c_void,
                                        dc: *mut ::libc::c_void,
                                        y: ::libc::c_int,
                                        start: ::libc::c_int,
                                        end: ::libc::c_int) -> ();
    pub fn wxPlotOnOffCurve_DrawOnLine(_obj: *mut ::libc::c_void,
                                       dc: *mut ::libc::c_void,
                                       y: ::libc::c_int, start: ::libc::c_int,
                                       end: ::libc::c_int,
                                       clientData: *mut ::libc::c_void) -> ();
    pub fn wxPlotOnOffCurve_GetAt(_obj: *mut ::libc::c_void,
                                  index: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPlotOnOffCurve_GetClientData(_obj: *mut ::libc::c_void,
                                          index: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPlotOnOffCurve_GetCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotOnOffCurve_GetEndX(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotOnOffCurve_GetOff(_obj: *mut ::libc::c_void,
                                   index: ::libc::c_int) -> ::libc::c_int;
    pub fn wxPlotOnOffCurve_GetOffsetY(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotOnOffCurve_GetOn(_obj: *mut ::libc::c_void,
                                  index: ::libc::c_int) -> ::libc::c_int;
    pub fn wxPlotOnOffCurve_GetStartX(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotOnOffCurve_SetOffsetY(_obj: *mut ::libc::c_void,
                                       offsetY: ::libc::c_int) -> ();
    pub fn wxPlotWindow_Add(_obj: *mut ::libc::c_void,
                            curve: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_AddOnOff(_obj: *mut ::libc::c_void,
                                 curve: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_Create(parent: *mut ::libc::c_void, id: ::libc::c_int,
                               x: ::libc::c_int, y: ::libc::c_int,
                               w: ::libc::c_int, h: ::libc::c_int,
                               flags: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxPlotWindow_Delete(_obj: *mut ::libc::c_void,
                               curve: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_DeleteOnOff(_obj: *mut ::libc::c_void,
                                    curve: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_Enlarge(_obj: *mut ::libc::c_void,
                                curve: *mut ::libc::c_void,
                                factor: ::libc::c_double) -> ();
    pub fn wxPlotWindow_GetAt(_obj: *mut ::libc::c_void, n: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPlotWindow_GetCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPlotWindow_GetCurrent(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPlotWindow_GetEnlargeAroundWindowCentre(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotWindow_GetOnOffCurveAt(_obj: *mut ::libc::c_void,
                                        n: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPlotWindow_GetOnOffCurveCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotWindow_GetScrollOnThumbRelease(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPlotWindow_GetUnitsPerValue(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxPlotWindow_GetZoom(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxPlotWindow_Move(_obj: *mut ::libc::c_void,
                             curve: *mut ::libc::c_void,
                             pixels_up: ::libc::c_int) -> ();
    pub fn wxPlotWindow_RedrawEverything(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_RedrawXAxis(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_RedrawYAxis(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_ResetScrollbar(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_SetCurrent(_obj: *mut ::libc::c_void,
                                   current: *mut ::libc::c_void) -> ();
    pub fn wxPlotWindow_SetEnlargeAroundWindowCentre(_obj:
                                                         *mut ::libc::c_void,
                                                     enlargeAroundWindowCentre:
                                                         ::libc::c_int) -> ();
    pub fn wxPlotWindow_SetScrollOnThumbRelease(_obj: *mut ::libc::c_void,
                                                scrollOnThumbRelease:
                                                    ::libc::c_int) -> ();
    pub fn wxPlotWindow_SetUnitsPerValue(_obj: *mut ::libc::c_void,
                                         upv: ::libc::c_double) -> ();
    pub fn wxPlotWindow_SetZoom(_obj: *mut ::libc::c_void,
                                zoom: ::libc::c_double) -> ();
    pub fn wxPoint_Create(xx: ::libc::c_int, yy: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPoint_Destroy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPoint_GetX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPoint_GetY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPoint_SetX(_obj: *mut ::libc::c_void, w: ::libc::c_int) -> ();
    pub fn wxPoint_SetY(_obj: *mut ::libc::c_void, h: ::libc::c_int) -> ();
    pub fn wxPostScriptDC_Create(data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPostScriptDC_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxPostScriptDC_SetResolution(_self: *mut ::libc::c_void,
                                        ppi: ::libc::c_int) -> ();
    pub fn wxPostScriptDC_GetResolution(_self: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPreviewCanvas_Create(preview: *mut ::libc::c_void,
                                  parent: *mut ::libc::c_void,
                                  x: ::libc::c_int, y: ::libc::c_int,
                                  w: ::libc::c_int, h: ::libc::c_int,
                                  style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_Assign(_obj: *mut ::libc::c_void,
                              data: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_Create() -> *mut ::libc::c_void;
    pub fn wxPrintData_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_GetCollate(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintData_GetColour(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintData_GetDuplex(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintData_GetFilename(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetFontMetricPath(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetNoCopies(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintData_GetOrientation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintData_GetPaperId(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintData_GetPaperSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetPreviewCommand(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetPrintMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintData_GetPrinterCommand(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetPrinterName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetPrinterOptions(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintData_GetPrinterScaleX(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxPrintData_GetPrinterScaleY(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxPrintData_GetPrinterTranslateX(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintData_GetPrinterTranslateY(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintData_GetQuality(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintData_SetCollate(_obj: *mut ::libc::c_void,
                                  flag: ::libc::c_int) -> ();
    pub fn wxPrintData_SetColour(_obj: *mut ::libc::c_void,
                                 colour: ::libc::c_int) -> ();
    pub fn wxPrintData_SetDuplex(_obj: *mut ::libc::c_void,
                                 duplex: ::libc::c_int) -> ();
    pub fn wxPrintData_SetFilename(_obj: *mut ::libc::c_void,
                                   filename: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_SetFontMetricPath(_obj: *mut ::libc::c_void,
                                         path: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_SetNoCopies(_obj: *mut ::libc::c_void,
                                   v: ::libc::c_int) -> ();
    pub fn wxPrintData_SetOrientation(_obj: *mut ::libc::c_void,
                                      orient: ::libc::c_int) -> ();
    pub fn wxPrintData_SetPaperId(_obj: *mut ::libc::c_void,
                                  sizeId: ::libc::c_int) -> ();
    pub fn wxPrintData_SetPaperSize(_obj: *mut ::libc::c_void,
                                    w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxPrintData_SetPreviewCommand(_obj: *mut ::libc::c_void,
                                         command: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_SetPrintMode(_obj: *mut ::libc::c_void,
                                    printMode: ::libc::c_int) -> ();
    pub fn wxPrintData_SetPrinterCommand(_obj: *mut ::libc::c_void,
                                         command: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_SetPrinterName(_obj: *mut ::libc::c_void,
                                      name: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_SetPrinterOptions(_obj: *mut ::libc::c_void,
                                         options: *mut ::libc::c_void) -> ();
    pub fn wxPrintData_SetPrinterScaleX(_obj: *mut ::libc::c_void,
                                        x: ::libc::c_double) -> ();
    pub fn wxPrintData_SetPrinterScaleY(_obj: *mut ::libc::c_void,
                                        y: ::libc::c_double) -> ();
    pub fn wxPrintData_SetPrinterScaling(_obj: *mut ::libc::c_void,
                                         x: ::libc::c_double,
                                         y: ::libc::c_double) -> ();
    pub fn wxPrintData_SetPrinterTranslateX(_obj: *mut ::libc::c_void,
                                            x: ::libc::c_int) -> ();
    pub fn wxPrintData_SetPrinterTranslateY(_obj: *mut ::libc::c_void,
                                            y: ::libc::c_int) -> ();
    pub fn wxPrintData_SetPrinterTranslation(_obj: *mut ::libc::c_void,
                                             x: ::libc::c_int,
                                             y: ::libc::c_int) -> ();
    pub fn wxPrintData_SetQuality(_obj: *mut ::libc::c_void,
                                  quality: ::libc::c_int) -> ();
    pub fn wxPostScriptPrintNativeData_Create() -> *mut ::libc::c_void;
    pub fn wxPostScriptPrintNativeData_Delete(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxPrintDialog_Create(parent: *mut ::libc::c_void,
                                data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintDialog_GetPrintDC(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintDialog_GetPrintData(_obj: *mut ::libc::c_void,
                                      _ref: *mut ::libc::c_void) -> ();
    pub fn wxPrintDialog_GetPrintDialogData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintDialogData_Assign(_obj: *mut ::libc::c_void,
                                    data: *mut ::libc::c_void) -> ();
    pub fn wxPrintDialogData_AssignData(_obj: *mut ::libc::c_void,
                                        data: *mut ::libc::c_void) -> ();
    pub fn wxPrintDialogData_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxPrintDialogData_CreateFromData(printData: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintDialogData_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPrintDialogData_EnableHelp(_obj: *mut ::libc::c_void,
                                        flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_EnablePageNumbers(_obj: *mut ::libc::c_void,
                                               flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_EnablePrintToFile(_obj: *mut ::libc::c_void,
                                               flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_EnableSelection(_obj: *mut ::libc::c_void,
                                             flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_GetAllPages(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetCollate(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetEnableHelp(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetEnablePageNumbers(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetEnablePrintToFile(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetEnableSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetFromPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetMaxPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetMinPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetNoCopies(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetPrintData(_obj: *mut ::libc::c_void,
                                          _ref: *mut ::libc::c_void) -> ();
    pub fn wxPrintDialogData_GetPrintToFile(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_GetToPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintDialogData_SetAllPages(_obj: *mut ::libc::c_void,
                                         flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetCollate(_obj: *mut ::libc::c_void,
                                        flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetFromPage(_obj: *mut ::libc::c_void,
                                         v: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetMaxPage(_obj: *mut ::libc::c_void,
                                        v: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetMinPage(_obj: *mut ::libc::c_void,
                                        v: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetNoCopies(_obj: *mut ::libc::c_void,
                                         v: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetPrintData(_obj: *mut ::libc::c_void,
                                          printData: *mut ::libc::c_void)
     -> ();
    pub fn wxPrintDialogData_SetPrintToFile(_obj: *mut ::libc::c_void,
                                            flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetSelection(_obj: *mut ::libc::c_void,
                                          flag: ::libc::c_int) -> ();
    pub fn wxPrintDialogData_SetToPage(_obj: *mut ::libc::c_void,
                                       v: ::libc::c_int) -> ();
    pub fn wxPrintPreview_CreateFromData(printout: *mut ::libc::c_void,
                                         printoutForPrinting:
                                             *mut ::libc::c_void,
                                         data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintPreview_CreateFromDialogData(printout: *mut ::libc::c_void,
                                               printoutForPrinting:
                                                   *mut ::libc::c_void,
                                               data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintPreview_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPrintPreview_DetermineScaling(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPrintPreview_DrawBlankPage(_obj: *mut ::libc::c_void,
                                        canvas: *mut ::libc::c_void,
                                        dc: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintPreview_GetCanvas(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintPreview_GetCurrentPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintPreview_GetFrame(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintPreview_GetMaxPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintPreview_GetMinPage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPrintPreview_GetPrintDialogData(_obj: *mut ::libc::c_void,
                                             _ref: *mut ::libc::c_void) -> ();
    pub fn wxPrintPreview_GetPrintout(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintPreview_GetPrintoutForPrinting(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintPreview_GetZoom(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintPreview_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintPreview_PaintPage(_obj: *mut ::libc::c_void,
                                    canvas: *mut ::libc::c_void,
                                    dc: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintPreview_Print(_obj: *mut ::libc::c_void,
                                interactive: ::libc::c_int) -> ::libc::c_int;
    pub fn wxPrintPreview_RenderPage(_obj: *mut ::libc::c_void,
                                     pageNum: ::libc::c_int) -> ::libc::c_int;
    pub fn wxPrintPreview_SetCanvas(_obj: *mut ::libc::c_void,
                                    canvas: *mut ::libc::c_void) -> ();
    pub fn wxPrintPreview_SetCurrentPage(_obj: *mut ::libc::c_void,
                                         pageNum: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxPrintPreview_SetFrame(_obj: *mut ::libc::c_void,
                                   frame: *mut ::libc::c_void) -> ();
    pub fn wxPrintPreview_SetOk(_obj: *mut ::libc::c_void, ok: ::libc::c_int)
     -> ();
    pub fn wxPrintPreview_SetPrintout(_obj: *mut ::libc::c_void,
                                      printout: *mut ::libc::c_void) -> ();
    pub fn wxPrintPreview_SetZoom(_obj: *mut ::libc::c_void,
                                  percent: ::libc::c_int) -> ();
    pub fn wxPrinter_Create(data: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxPrinter_CreateAbortWindow(_obj: *mut ::libc::c_void,
                                       parent: *mut ::libc::c_void,
                                       printout: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrinter_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxPrinter_GetAbort(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrinter_GetLastError(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrinter_GetPrintDialogData(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxPrinter_Print(_obj: *mut ::libc::c_void,
                           parent: *mut ::libc::c_void,
                           printout: *mut ::libc::c_void,
                           prompt: ::libc::c_int) -> ::libc::c_int;
    pub fn wxPrinter_PrintDialog(_obj: *mut ::libc::c_void,
                                 parent: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrinter_ReportError(_obj: *mut ::libc::c_void,
                                 parent: *mut ::libc::c_void,
                                 printout: *mut ::libc::c_void,
                                 message: *mut ::libc::c_void) -> ();
    pub fn wxPrinter_Setup(_obj: *mut ::libc::c_void,
                           parent: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrinterDC_Create(data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrinterDC_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxPrinterDC_GetPaperRect(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxProcess_CloseOutput(_obj: *mut ::libc::c_void) -> ();
    pub fn wxProcess_CreateDefault(_prt: *mut ::libc::c_void,
                                   _id: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxProcess_CreateRedirect(_prt: *mut ::libc::c_void,
                                    _rdr: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxProcess_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxProcess_Detach(_obj: *mut ::libc::c_void) -> ();
    pub fn wxProcess_GetErrorStream(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxProcess_GetInputStream(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxProcess_GetOutputStream(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxProcess_IsRedirected(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxProcess_Redirect(_obj: *mut ::libc::c_void) -> ();
    pub fn wxProcessEvent_GetExitCode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxProcessEvent_GetPid(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxQueryLayoutInfoEvent_Create(id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxQueryLayoutInfoEvent_GetAlignment(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxQueryLayoutInfoEvent_GetFlags(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxQueryLayoutInfoEvent_GetOrientation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxQueryLayoutInfoEvent_GetRequestedLength(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxQueryLayoutInfoEvent_GetSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxQueryLayoutInfoEvent_SetAlignment(_obj: *mut ::libc::c_void,
                                               align: ::libc::c_int) -> ();
    pub fn wxQueryLayoutInfoEvent_SetFlags(_obj: *mut ::libc::c_void,
                                           flags: ::libc::c_int) -> ();
    pub fn wxQueryLayoutInfoEvent_SetOrientation(_obj: *mut ::libc::c_void,
                                                 orient: ::libc::c_int) -> ();
    pub fn wxQueryLayoutInfoEvent_SetRequestedLength(_obj:
                                                         *mut ::libc::c_void,
                                                     length: ::libc::c_int)
     -> ();
    pub fn wxQueryLayoutInfoEvent_SetSize(_obj: *mut ::libc::c_void,
                                          w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxQueryNewPaletteEvent_CopyObject(_obj: *mut ::libc::c_void,
                                             obj: *mut ::libc::c_void) -> ();
    pub fn wxQueryNewPaletteEvent_GetPaletteRealized(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxQueryNewPaletteEvent_SetPaletteRealized(_obj:
                                                         *mut ::libc::c_void,
                                                     realized: ::libc::c_int)
     -> ();
    pub fn wxRadioBox_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                             _top: ::libc::c_int, _wdt: ::libc::c_int,
                             _hgt: ::libc::c_int, n: ::libc::c_int,
                             _str: *mut *mut ::libc::c_char,
                             _dim: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxRadioBox_EnableItem(_obj: *mut ::libc::c_void,
                                 item: ::libc::c_int, enable: ::libc::c_int)
     -> ();
    pub fn wxRadioBox_FindString(_obj: *mut ::libc::c_void,
                                 s: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRadioBox_GetItemLabel(_obj: *mut ::libc::c_void,
                                   item: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxRadioBox_GetNumberOfRowsOrCols(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRadioBox_GetSelection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRadioBox_GetStringSelection(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxRadioBox_Number(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRadioBox_SetItemBitmap(_obj: *mut ::libc::c_void,
                                    item: ::libc::c_int,
                                    bitmap: *mut ::libc::c_void) -> ();
    pub fn wxRadioBox_SetItemLabel(_obj: *mut ::libc::c_void,
                                   item: ::libc::c_int,
                                   label: *mut ::libc::c_void) -> ();
    pub fn wxRadioBox_SetNumberOfRowsOrCols(_obj: *mut ::libc::c_void,
                                            n: ::libc::c_int) -> ();
    pub fn wxRadioBox_SetSelection(_obj: *mut ::libc::c_void,
                                   _n: ::libc::c_int) -> ();
    pub fn wxRadioBox_SetStringSelection(_obj: *mut ::libc::c_void,
                                         s: *mut ::libc::c_void) -> ();
    pub fn wxRadioBox_ShowItem(_obj: *mut ::libc::c_void, item: ::libc::c_int,
                               show: ::libc::c_int) -> ();
    pub fn wxRadioButton_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                                _txt: *mut ::libc::c_void,
                                _lft: ::libc::c_int, _top: ::libc::c_int,
                                _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxRadioButton_GetValue(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRadioButton_SetValue(_obj: *mut ::libc::c_void,
                                  value: ::libc::c_int) -> ();
    pub fn wxRegion_Assign(_obj: *mut ::libc::c_void,
                           region: *mut ::libc::c_void) -> ();
    pub fn wxRegion_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxRegion_ContainsPoint(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                  y: ::libc::c_int) -> ::libc::c_int;
    pub fn wxRegion_ContainsRect(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int, width: ::libc::c_int,
                                 height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxRegion_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxRegion_CreateFromRect(x: ::libc::c_int, y: ::libc::c_int,
                                   w: ::libc::c_int, h: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxRegion_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxRegion_IsEmpty(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRegion_GetBox(_obj: *mut ::libc::c_void, _x: *mut ::libc::c_void,
                           _y: *mut ::libc::c_void, _w: *mut ::libc::c_void,
                           _h: *mut ::libc::c_void) -> ();
    pub fn wxRegion_IntersectRect(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                  y: ::libc::c_int, width: ::libc::c_int,
                                  height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxRegion_IntersectRegion(_obj: *mut ::libc::c_void,
                                    region: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRegion_SubtractRect(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int, width: ::libc::c_int,
                                 height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxRegion_SubtractRegion(_obj: *mut ::libc::c_void,
                                   region: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRegion_UnionRect(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, width: ::libc::c_int,
                              height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxRegion_UnionRegion(_obj: *mut ::libc::c_void,
                                region: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRegion_XorRect(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                            y: ::libc::c_int, width: ::libc::c_int,
                            height: ::libc::c_int) -> ::libc::c_int;
    pub fn wxRegion_XorRegion(_obj: *mut ::libc::c_void,
                              region: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRegionIterator_Create() -> *mut ::libc::c_void;
    pub fn wxRegionIterator_CreateFromRegion(region: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxRegionIterator_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxRegionIterator_GetHeight(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRegionIterator_GetWidth(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRegionIterator_GetX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRegionIterator_GetY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxRegionIterator_HaveRects(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxRegionIterator_Next(_obj: *mut ::libc::c_void) -> ();
    pub fn wxRegionIterator_Reset(_obj: *mut ::libc::c_void) -> ();
    pub fn wxRegionIterator_ResetToRegion(_obj: *mut ::libc::c_void,
                                          region: *mut ::libc::c_void) -> ();
    pub fn wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars(_obj:
                                                                 *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_CalcTreeSize(_obj: *mut ::libc::c_void,
                                                   _x: *mut ::libc::c_void,
                                                   _y: *mut ::libc::c_void,
                                                   _w: *mut ::libc::c_void,
                                                   _h: *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem(_obj:
                                                           *mut ::libc::c_void,
                                                       id:
                                                           *mut ::libc::c_void,
                                                       _x:
                                                           *mut ::libc::c_void,
                                                       _y:
                                                           *mut ::libc::c_void,
                                                       _w:
                                                           *mut ::libc::c_void,
                                                       _h:
                                                           *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_Create(_obj: *mut ::libc::c_void,
                                             _cmp: *mut ::libc::c_void,
                                             parent: *mut ::libc::c_void,
                                             id: ::libc::c_int,
                                             x: ::libc::c_int,
                                             y: ::libc::c_int,
                                             w: ::libc::c_int,
                                             h: ::libc::c_int,
                                             style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxRemotelyScrolledTreeCtrl_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxRemotelyScrolledTreeCtrl_GetCompanionWindow(_obj:
                                                             *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxRemotelyScrolledTreeCtrl_GetScrollPos(_obj: *mut ::libc::c_void,
                                                   orient: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxRemotelyScrolledTreeCtrl_GetScrolledWindow(_obj:
                                                            *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxRemotelyScrolledTreeCtrl_GetViewStart(_obj: *mut ::libc::c_void,
                                                   _x: *mut ::libc::c_void,
                                                   _y: *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_HideVScrollbar(_obj:
                                                         *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_PrepareDC(_obj: *mut ::libc::c_void,
                                                dc: *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_ScrollToLine(_obj: *mut ::libc::c_void,
                                                   posHoriz: ::libc::c_int,
                                                   posVert: ::libc::c_int)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_SetCompanionWindow(_obj:
                                                             *mut ::libc::c_void,
                                                         companion:
                                                             *mut ::libc::c_void)
     -> ();
    pub fn wxRemotelyScrolledTreeCtrl_SetScrollbars(_obj: *mut ::libc::c_void,
                                                    pixelsPerUnitX:
                                                        ::libc::c_int,
                                                    pixelsPerUnitY:
                                                        ::libc::c_int,
                                                    noUnitsX: ::libc::c_int,
                                                    noUnitsY: ::libc::c_int,
                                                    xPos: ::libc::c_int,
                                                    yPos: ::libc::c_int,
                                                    noRefresh: ::libc::c_int)
     -> ();
    pub fn wxSVGFileDC_Create(fileName: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSVGFileDC_CreateWithSize(fileName: *mut ::libc::c_void,
                                      w: ::libc::c_int, h: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSVGFileDC_CreateWithSizeAndResolution(fileName:
                                                       *mut ::libc::c_void,
                                                   w: ::libc::c_int,
                                                   h: ::libc::c_int,
                                                   a_dpi: ::libc::c_float)
     -> *mut ::libc::c_void;
    pub fn wxSVGFileDC_Delete(obj: *mut ::libc::c_void) -> ();
    pub fn wxSashEvent_Create(id: ::libc::c_int, edge: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSashEvent_GetDragRect(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSashEvent_GetDragStatus(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashEvent_GetEdge(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSashEvent_SetDragRect(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int,
                                   w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxSashEvent_SetDragStatus(_obj: *mut ::libc::c_void,
                                     status: ::libc::c_int) -> ();
    pub fn wxSashEvent_SetEdge(_obj: *mut ::libc::c_void, edge: ::libc::c_int)
     -> ();
    pub fn wxSashLayoutWindow_Create(_par: *mut ::libc::c_void,
                                     _id: ::libc::c_int, _x: ::libc::c_int,
                                     _y: ::libc::c_int, _w: ::libc::c_int,
                                     _h: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSashLayoutWindow_GetAlignment(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashLayoutWindow_GetOrientation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashLayoutWindow_SetAlignment(_obj: *mut ::libc::c_void,
                                           align: ::libc::c_int) -> ();
    pub fn wxSashLayoutWindow_SetDefaultSize(_obj: *mut ::libc::c_void,
                                             w: ::libc::c_int,
                                             h: ::libc::c_int) -> ();
    pub fn wxSashLayoutWindow_SetOrientation(_obj: *mut ::libc::c_void,
                                             orient: ::libc::c_int) -> ();
    pub fn wxSashWindow_Create(_par: *mut ::libc::c_void, _id: ::libc::c_int,
                               _x: ::libc::c_int, _y: ::libc::c_int,
                               _w: ::libc::c_int, _h: ::libc::c_int,
                               _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxSashWindow_GetDefaultBorderSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashWindow_GetEdgeMargin(_obj: *mut ::libc::c_void,
                                      edge: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSashWindow_GetExtraBorderSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashWindow_GetMaximumSizeX(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashWindow_GetMaximumSizeY(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashWindow_GetMinimumSizeX(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashWindow_GetMinimumSizeY(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSashWindow_GetSashVisible(_obj: *mut ::libc::c_void,
                                       edge: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSashWindow_HasBorder(_obj: *mut ::libc::c_void,
                                  edge: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSashWindow_SetDefaultBorderSize(_obj: *mut ::libc::c_void,
                                             width: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetExtraBorderSize(_obj: *mut ::libc::c_void,
                                           width: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetMaximumSizeX(_obj: *mut ::libc::c_void,
                                        max: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetMaximumSizeY(_obj: *mut ::libc::c_void,
                                        max: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetMinimumSizeX(_obj: *mut ::libc::c_void,
                                        min: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetMinimumSizeY(_obj: *mut ::libc::c_void,
                                        min: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetSashBorder(_obj: *mut ::libc::c_void,
                                      edge: ::libc::c_int,
                                      border: ::libc::c_int) -> ();
    pub fn wxSashWindow_SetSashVisible(_obj: *mut ::libc::c_void,
                                       edge: ::libc::c_int,
                                       sash: ::libc::c_int) -> ();
    pub fn wxScreenDC_Create() -> *mut ::libc::c_void;
    pub fn wxScreenDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxScreenDC_EndDrawingOnTop(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScreenDC_StartDrawingOnTop(_obj: *mut ::libc::c_void,
                                        x: ::libc::c_int, y: ::libc::c_int,
                                        w: ::libc::c_int, h: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxScreenDC_StartDrawingOnTopOfWin(_obj: *mut ::libc::c_void,
                                             win: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollBar_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                              _lft: ::libc::c_int, _top: ::libc::c_int,
                              _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                              _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxScrollBar_GetPageSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollBar_GetRange(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxScrollBar_GetThumbPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollBar_GetThumbSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollBar_SetScrollbar(_obj: *mut ::libc::c_void,
                                    position: ::libc::c_int,
                                    thumbSize: ::libc::c_int,
                                    range: ::libc::c_int,
                                    pageSize: ::libc::c_int,
                                    refresh: ::libc::c_int) -> ();
    pub fn wxScrollBar_SetThumbPosition(_obj: *mut ::libc::c_void,
                                        viewStart: ::libc::c_int) -> ();
    pub fn wxScrollEvent_GetOrientation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollWinEvent_GetOrientation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollWinEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxScrollWinEvent_SetOrientation(_obj: *mut ::libc::c_void,
                                           orient: ::libc::c_int) -> ();
    pub fn wxScrollWinEvent_SetPosition(_obj: *mut ::libc::c_void,
                                        pos: ::libc::c_int) -> ();
    pub fn wxScrolledWindow_AdjustScrollbars(_obj: *mut ::libc::c_void) -> ();
    pub fn wxScrolledWindow_CalcScrolledPosition(_obj: *mut ::libc::c_void,
                                                 x: ::libc::c_int,
                                                 y: ::libc::c_int,
                                                 xx: *mut ::libc::c_void,
                                                 yy: *mut ::libc::c_void)
     -> ();
    pub fn wxScrolledWindow_CalcUnscrolledPosition(_obj: *mut ::libc::c_void,
                                                   x: ::libc::c_int,
                                                   y: ::libc::c_int,
                                                   xx: *mut ::libc::c_void,
                                                   yy: *mut ::libc::c_void)
     -> ();
    pub fn wxScrolledWindow_Create(_prt: *mut ::libc::c_void,
                                   _id: ::libc::c_int, _lft: ::libc::c_int,
                                   _top: ::libc::c_int, _wdt: ::libc::c_int,
                                   _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxScrolledWindow_EnableScrolling(_obj: *mut ::libc::c_void,
                                            x_scrolling: ::libc::c_int,
                                            y_scrolling: ::libc::c_int) -> ();
    pub fn wxScrolledWindow_GetScaleX(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxScrolledWindow_GetScaleY(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxScrolledWindow_GetScrollPageSize(_obj: *mut ::libc::c_void,
                                              orient: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxScrolledWindow_GetScrollPixelsPerUnit(_obj: *mut ::libc::c_void,
                                                   _x: *mut ::libc::c_void,
                                                   _y: *mut ::libc::c_void)
     -> ();
    pub fn wxScrolledWindow_GetTargetWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxScrolledWindow_GetViewStart(_obj: *mut ::libc::c_void,
                                         _x: *mut ::libc::c_void,
                                         _y: *mut ::libc::c_void) -> ();
    pub fn wxScrolledWindow_GetVirtualSize(_obj: *mut ::libc::c_void,
                                           _x: *mut ::libc::c_void,
                                           _y: *mut ::libc::c_void) -> ();
    pub fn wxScrolledWindow_OnDraw(_obj: *mut ::libc::c_void,
                                   dc: *mut ::libc::c_void) -> ();
    pub fn wxScrolledWindow_PrepareDC(_obj: *mut ::libc::c_void,
                                      dc: *mut ::libc::c_void) -> ();
    pub fn wxScrolledWindow_Scroll(_obj: *mut ::libc::c_void,
                                   x_pos: ::libc::c_int, y_pos: ::libc::c_int)
     -> ();
    pub fn wxScrolledWindow_SetScale(_obj: *mut ::libc::c_void,
                                     xs: ::libc::c_double,
                                     ys: ::libc::c_double) -> ();
    pub fn wxScrolledWindow_SetScrollPageSize(_obj: *mut ::libc::c_void,
                                              orient: ::libc::c_int,
                                              pageSize: ::libc::c_int) -> ();
    pub fn wxScrolledWindow_SetScrollbars(_obj: *mut ::libc::c_void,
                                          pixelsPerUnitX: ::libc::c_int,
                                          pixelsPerUnitY: ::libc::c_int,
                                          noUnitsX: ::libc::c_int,
                                          noUnitsY: ::libc::c_int,
                                          xPos: ::libc::c_int,
                                          yPos: ::libc::c_int,
                                          noRefresh: ::libc::c_int) -> ();
    pub fn wxScrolledWindow_ShowScrollbars(_obj: *mut ::libc::c_void,
                                           showh: ::libc::c_int,
                                           showv: ::libc::c_int) -> ();
    pub fn wxScrolledWindow_SetTargetWindow(_obj: *mut ::libc::c_void,
                                            target: *mut ::libc::c_void)
     -> ();
    pub fn wxScrolledWindow_ViewStart(_obj: *mut ::libc::c_void,
                                      _x: *mut ::libc::c_void,
                                      _y: *mut ::libc::c_void) -> ();
    pub fn wxSetCursorEvent_GetCursor(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSetCursorEvent_GetX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSetCursorEvent_GetY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSetCursorEvent_HasCursor(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSetCursorEvent_SetCursor(_obj: *mut ::libc::c_void,
                                      cursor: *mut ::libc::c_void) -> ();
    pub fn wxShowEvent_CopyObject(_obj: *mut ::libc::c_void,
                                  obj: *mut ::libc::c_void) -> ();
    pub fn wxShowEvent_IsShown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxShowEvent_SetShow(_obj: *mut ::libc::c_void, show: ::libc::c_int)
     -> ();
    pub fn wxSimpleHelpProvider_Create() -> *mut ::libc::c_void;
    pub fn wxSingleInstanceChecker_Create(_obj: *mut ::libc::c_void,
                                          name: *mut ::libc::c_void,
                                          path: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSingleInstanceChecker_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxSingleInstanceChecker_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSingleInstanceChecker_IsAnotherRunning(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSize_Create(w: ::libc::c_int, h: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSize_Destroy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSize_GetHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSize_GetWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSize_SetHeight(_obj: *mut ::libc::c_void, h: ::libc::c_int)
     -> ();
    pub fn wxSize_SetWidth(_obj: *mut ::libc::c_void, w: ::libc::c_int) -> ();
    pub fn wxSizeEvent_CopyObject(_obj: *mut ::libc::c_void,
                                  obj: *mut ::libc::c_void) -> ();
    pub fn wxSizeEvent_GetSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizer_Add(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                       height: ::libc::c_int, option: ::libc::c_int,
                       flag: ::libc::c_int, border: ::libc::c_int,
                       userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_AddSizer(_obj: *mut ::libc::c_void,
                            sizer: *mut ::libc::c_void, option: ::libc::c_int,
                            flag: ::libc::c_int, border: ::libc::c_int,
                            userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_AddWindow(_obj: *mut ::libc::c_void,
                             window: *mut ::libc::c_void,
                             option: ::libc::c_int, flag: ::libc::c_int,
                             border: ::libc::c_int,
                             userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_CalcMin(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxSizer_Fit(_obj: *mut ::libc::c_void, window: *mut ::libc::c_void)
     -> ();
    pub fn wxSizer_GetChildren(_obj: *mut ::libc::c_void,
                               _res: *mut ::libc::c_void, _cnt: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSizer_GetMinSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizer_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizer_GetSize(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxSizer_Insert(_obj: *mut ::libc::c_void, before: ::libc::c_int,
                          width: ::libc::c_int, height: ::libc::c_int,
                          option: ::libc::c_int, flag: ::libc::c_int,
                          border: ::libc::c_int,
                          userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_InsertSizer(_obj: *mut ::libc::c_void,
                               before: ::libc::c_int,
                               sizer: *mut ::libc::c_void,
                               option: ::libc::c_int, flag: ::libc::c_int,
                               border: ::libc::c_int,
                               userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_InsertWindow(_obj: *mut ::libc::c_void,
                                before: ::libc::c_int,
                                window: *mut ::libc::c_void,
                                option: ::libc::c_int, flag: ::libc::c_int,
                                border: ::libc::c_int,
                                userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_Layout(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSizer_Prepend(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                           height: ::libc::c_int, option: ::libc::c_int,
                           flag: ::libc::c_int, border: ::libc::c_int,
                           userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_PrependSizer(_obj: *mut ::libc::c_void,
                                sizer: *mut ::libc::c_void,
                                option: ::libc::c_int, flag: ::libc::c_int,
                                border: ::libc::c_int,
                                userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_PrependWindow(_obj: *mut ::libc::c_void,
                                 window: *mut ::libc::c_void,
                                 option: ::libc::c_int, flag: ::libc::c_int,
                                 border: ::libc::c_int,
                                 userData: *mut ::libc::c_void) -> ();
    pub fn wxSizer_RecalcSizes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSizer_SetDimension(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int, width: ::libc::c_int,
                                height: ::libc::c_int) -> ();
    pub fn wxSizer_SetItemMinSize(_obj: *mut ::libc::c_void,
                                  pos: ::libc::c_int, width: ::libc::c_int,
                                  height: ::libc::c_int) -> ();
    pub fn wxSizer_SetItemMinSizeSizer(_obj: *mut ::libc::c_void,
                                       sizer: *mut ::libc::c_void,
                                       width: ::libc::c_int,
                                       height: ::libc::c_int) -> ();
    pub fn wxSizer_SetItemMinSizeWindow(_obj: *mut ::libc::c_void,
                                        window: *mut ::libc::c_void,
                                        width: ::libc::c_int,
                                        height: ::libc::c_int) -> ();
    pub fn wxSizer_SetMinSize(_obj: *mut ::libc::c_void, width: ::libc::c_int,
                              height: ::libc::c_int) -> ();
    pub fn wxSizer_SetSizeHints(_obj: *mut ::libc::c_void,
                                window: *mut ::libc::c_void) -> ();
    pub fn wxSizer_AddSpacer(_obj: *mut ::libc::c_void, size: ::libc::c_int)
     -> ();
    pub fn wxSizer_AddStretchSpacer(_obj: *mut ::libc::c_void,
                                    size: ::libc::c_int) -> ();
    pub fn wxSizer_Clear(_obj: *mut ::libc::c_void,
                         delete_windows: ::libc::c_int) -> ();
    pub fn wxSizer_DetachWindow(_obj: *mut ::libc::c_void,
                                window: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizer_DetachSizer(_obj: *mut ::libc::c_void,
                               sizer: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizer_Detach(_obj: *mut ::libc::c_void, index: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSizer_FitInside(_obj: *mut ::libc::c_void,
                             window: *mut ::libc::c_void) -> ();
    pub fn wxSizer_GetContainingWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizer_GetItemWindow(_obj: *mut ::libc::c_void,
                                 window: *mut ::libc::c_void,
                                 recursive: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSizer_GetItemSizer(_obj: *mut ::libc::c_void,
                                window: *mut ::libc::c_void,
                                recursive: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSizer_GetItem(_obj: *mut ::libc::c_void, index: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSizer_HideWindow(_obj: *mut ::libc::c_void,
                              window: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizer_HideSizer(_obj: *mut ::libc::c_void,
                             sizer: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizer_Hide(_obj: *mut ::libc::c_void, index: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSizer_InsertSpacer(_obj: *mut ::libc::c_void,
                                index: ::libc::c_int, size: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSizer_InsertStretchSpacer(_obj: *mut ::libc::c_void,
                                       index: ::libc::c_int,
                                       prop: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSizer_IsShownWindow(_obj: *mut ::libc::c_void,
                                 window: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSizer_IsShownSizer(_obj: *mut ::libc::c_void,
                                sizer: *mut *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSizer_IsShown(_obj: *mut ::libc::c_void, index: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSizer_PrependSpacer(_obj: *mut ::libc::c_void,
                                 size: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxSizer_PrependStretchSpacer(_obj: *mut ::libc::c_void,
                                        prop: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSizer_ReplaceWindow(_obj: *mut ::libc::c_void,
                                 oldwin: *mut ::libc::c_void,
                                 newwin: *mut ::libc::c_void,
                                 recursive: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSizer_ReplaceSizer(_obj: *mut ::libc::c_void,
                                oldsz: *mut ::libc::c_void,
                                newsz: *mut ::libc::c_void,
                                recursive: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSizer_Replace(_obj: *mut ::libc::c_void, oldindex: ::libc::c_int,
                           newitem: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizer_SetVirtualSizeHints(_obj: *mut ::libc::c_void,
                                       window: *mut ::libc::c_void) -> ();
    pub fn wxSizer_ShowWindow(_obj: *mut ::libc::c_void,
                              window: *mut ::libc::c_void,
                              show: ::libc::c_int, recursive: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSizer_ShowSizer(_obj: *mut ::libc::c_void,
                             sizer: *mut ::libc::c_void, show: ::libc::c_int,
                             recursive: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSizer_Show(_obj: *mut ::libc::c_void, sizer: *mut ::libc::c_void,
                        index: ::libc::c_int, show: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSizerItem_CalcMin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_Create(width: ::libc::c_int, height: ::libc::c_int,
                              option: ::libc::c_int, flag: ::libc::c_int,
                              border: ::libc::c_int,
                              userData: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_CreateInSizer(sizer: *mut ::libc::c_void,
                                     option: ::libc::c_int,
                                     flag: ::libc::c_int,
                                     border: ::libc::c_int,
                                     userData: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_CreateInWindow(window: *mut ::libc::c_void,
                                      option: ::libc::c_int,
                                      flag: ::libc::c_int,
                                      border: ::libc::c_int,
                                      userData: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetBorder(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizerItem_GetFlag(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizerItem_GetMinSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetRatio(_obj: *mut ::libc::c_void) -> ::libc::c_float;
    pub fn wxSizerItem_GetSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetSizer(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetUserData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_IsSizer(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizerItem_IsSpacer(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizerItem_IsWindow(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizerItem_SetBorder(_obj: *mut ::libc::c_void,
                                 border: ::libc::c_int) -> ();
    pub fn wxSizerItem_SetDimension(_obj: *mut ::libc::c_void,
                                    _x: ::libc::c_int, _y: ::libc::c_int,
                                    _w: ::libc::c_int, _h: ::libc::c_int)
     -> ();
    pub fn wxSizerItem_SetFlag(_obj: *mut ::libc::c_void, flag: ::libc::c_int)
     -> ();
    pub fn wxSizerItem_SetFloatRatio(_obj: *mut ::libc::c_void,
                                     ratio: ::libc::c_float) -> ();
    pub fn wxSizerItem_SetInitSize(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxSizerItem_SetRatio(_obj: *mut ::libc::c_void,
                                width: ::libc::c_int, height: ::libc::c_int)
     -> ();
    pub fn wxSizerItem_SetSizer(_obj: *mut ::libc::c_void,
                                sizer: *mut ::libc::c_void) -> ();
    pub fn wxSizerItem_SetWindow(_obj: *mut ::libc::c_void,
                                 window: *mut ::libc::c_void) -> ();
    pub fn wxSizerItem_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSizerItem_DeleteWindows(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSizerItem_DetachSizer(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSizerItem_GetProportion(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSizerItem_GetRect(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_GetSpacer(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSizerItem_IsShown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSizerItem_SetProportion(_obj: *mut ::libc::c_void,
                                     proportion: ::libc::c_int) -> ();
    pub fn wxSizerItem_SetSpacer(_obj: *mut ::libc::c_void,
                                 width: ::libc::c_int, height: ::libc::c_int)
     -> ();
    pub fn wxSizerItem_Show(_obj: *mut ::libc::c_void, show: ::libc::c_int)
     -> ();
    pub fn wxSlider_ClearSel(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSlider_ClearTicks(_obj: *mut ::libc::c_void) -> ();
    pub fn wxSlider_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                           _init: ::libc::c_int, _min: ::libc::c_int,
                           _max: ::libc::c_int, _lft: ::libc::c_int,
                           _top: ::libc::c_int, _wdt: ::libc::c_int,
                           _hgt: ::libc::c_int, _stl: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxSlider_GetLineSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetPageSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetSelEnd(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetSelStart(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetThumbLength(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSlider_GetTickFreq(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_GetValue(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSlider_SetLineSize(_obj: *mut ::libc::c_void,
                                lineSize: ::libc::c_int) -> ();
    pub fn wxSlider_SetPageSize(_obj: *mut ::libc::c_void,
                                pageSize: ::libc::c_int) -> ();
    pub fn wxSlider_SetRange(_obj: *mut ::libc::c_void,
                             minValue: ::libc::c_int, maxValue: ::libc::c_int)
     -> ();
    pub fn wxSlider_SetSelection(_obj: *mut ::libc::c_void,
                                 minPos: ::libc::c_int, maxPos: ::libc::c_int)
     -> ();
    pub fn wxSlider_SetThumbLength(_obj: *mut ::libc::c_void,
                                   len: ::libc::c_int) -> ();
    pub fn wxSlider_SetTick(_obj: *mut ::libc::c_void, tickPos: ::libc::c_int)
     -> ();
    pub fn wxSlider_SetTickFreq(_obj: *mut ::libc::c_void, n: ::libc::c_int,
                                pos: ::libc::c_int) -> ();
    pub fn wxSlider_SetValue(_obj: *mut ::libc::c_void, value: ::libc::c_int)
     -> ();
    pub fn wxSpinButton_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                               _lft: ::libc::c_int, _top: ::libc::c_int,
                               _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                               _stl: ::libc::c_long) -> *mut ::libc::c_void;
    pub fn wxSpinButton_GetMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSpinButton_GetMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSpinButton_GetValue(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSpinButton_SetRange(_obj: *mut ::libc::c_void,
                                 minVal: ::libc::c_int, maxVal: ::libc::c_int)
     -> ();
    pub fn wxSpinButton_SetValue(_obj: *mut ::libc::c_void,
                                 val: ::libc::c_int) -> ();
    pub fn wxSpinCtrl_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                             _top: ::libc::c_int, _wdt: ::libc::c_int,
                             _hgt: ::libc::c_int, _stl: ::libc::c_long,
                             _min: ::libc::c_int, _max: ::libc::c_int,
                             _init: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxSpinCtrl_GetMax(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSpinCtrl_GetMin(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSpinCtrl_GetValue(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSpinCtrl_SetRange(_obj: *mut ::libc::c_void,
                               min_val: ::libc::c_int, max_val: ::libc::c_int)
     -> ();
    pub fn wxSpinCtrl_SetValue(_obj: *mut ::libc::c_void, val: ::libc::c_int)
     -> ();
    pub fn wxSpinEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSpinEvent_SetPosition(_obj: *mut ::libc::c_void,
                                   pos: ::libc::c_int) -> ();
    pub fn wxSplitterScrolledWindow_Create(parent: *mut ::libc::c_void,
                                           id: ::libc::c_int,
                                           x: ::libc::c_int, y: ::libc::c_int,
                                           w: ::libc::c_int, h: ::libc::c_int,
                                           style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSplitterWindow_Create(_prt: *mut ::libc::c_void,
                                   _id: ::libc::c_int, _lft: ::libc::c_int,
                                   _top: ::libc::c_int, _wdt: ::libc::c_int,
                                   _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxSplitterWindow_GetBorderSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_GetMinimumPaneSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_GetSashPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_GetSashSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_GetSplitMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_GetWindow1(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSplitterWindow_GetWindow2(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxSplitterWindow_Initialize(_obj: *mut ::libc::c_void,
                                       window: *mut ::libc::c_void) -> ();
    pub fn wxSplitterWindow_IsSplit(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_ReplaceWindow(_obj: *mut ::libc::c_void,
                                          winOld: *mut ::libc::c_void,
                                          winNew: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_SetBorderSize(_obj: *mut ::libc::c_void,
                                          width: ::libc::c_int) -> ();
    pub fn wxSplitterWindow_SetMinimumPaneSize(_obj: *mut ::libc::c_void,
                                               min: ::libc::c_int) -> ();
    pub fn wxSplitterWindow_SetSashPosition(_obj: *mut ::libc::c_void,
                                            position: ::libc::c_int,
                                            redraw: ::libc::c_int) -> ();
    pub fn wxSplitterWindow_SetSashSize(_obj: *mut ::libc::c_void,
                                        width: ::libc::c_int) -> ();
    pub fn wxSplitterWindow_SetSplitMode(_obj: *mut ::libc::c_void,
                                         mode: ::libc::c_int) -> ();
    pub fn wxSplitterWindow_SplitHorizontally(_obj: *mut ::libc::c_void,
                                              window1: *mut ::libc::c_void,
                                              window2: *mut ::libc::c_void,
                                              sashPosition: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_SplitVertically(_obj: *mut ::libc::c_void,
                                            window1: *mut ::libc::c_void,
                                            window2: *mut ::libc::c_void,
                                            sashPosition: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_Unsplit(_obj: *mut ::libc::c_void,
                                    toRemove: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxSplitterWindow_GetSashGravity(_obj: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxSplitterWindow_SetSashGravity(_obj: *mut ::libc::c_void,
                                           gravity: ::libc::c_double) -> ();
    pub fn wxStaticBitmap_Create(_prt: *mut ::libc::c_void,
                                 _id: ::libc::c_int,
                                 bitmap: *mut ::libc::c_void,
                                 _lft: ::libc::c_int, _top: ::libc::c_int,
                                 _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                 _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxStaticBitmap_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStaticBitmap_GetBitmap(_obj: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxStaticBitmap_GetIcon(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxStaticBitmap_SetBitmap(_obj: *mut ::libc::c_void,
                                    bitmap: *mut ::libc::c_void) -> ();
    pub fn wxStaticBitmap_SetIcon(_obj: *mut ::libc::c_void,
                                  icon: *mut ::libc::c_void) -> ();
    pub fn wxStaticBox_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                              _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                              _top: ::libc::c_int, _wdt: ::libc::c_int,
                              _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStaticBoxSizer_CalcMin(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStaticBoxSizer_Create(_box: *mut ::libc::c_void,
                                   orient: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStaticBoxSizer_GetStaticBox(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStaticBoxSizer_RecalcSizes(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStaticLine_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                               _lft: ::libc::c_int, _top: ::libc::c_int,
                               _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                               _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxStaticLine_GetDefaultSize(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStaticLine_IsVertical(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStaticText_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                               _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                               _top: ::libc::c_int, _wdt: ::libc::c_int,
                               _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStatusBar_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                              _lft: ::libc::c_int, _top: ::libc::c_int,
                              _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                              _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxStatusBar_GetBorderX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxStatusBar_GetBorderY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxStatusBar_GetFieldsCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStatusBar_GetStatusText(_obj: *mut ::libc::c_void,
                                     number: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStatusBar_SetFieldsCount(_obj: *mut ::libc::c_void,
                                      number: ::libc::c_int,
                                      widths: *mut ::libc::c_int) -> ();
    pub fn wxStatusBar_SetMinHeight(_obj: *mut ::libc::c_void,
                                    height: ::libc::c_int) -> ();
    pub fn wxStatusBar_SetStatusText(_obj: *mut ::libc::c_void,
                                     text: *mut ::libc::c_void,
                                     number: ::libc::c_int) -> ();
    pub fn wxStatusBar_SetStatusWidths(_obj: *mut ::libc::c_void,
                                       n: ::libc::c_int,
                                       widths: *mut ::libc::c_int) -> ();
    pub fn wxStopWatch_Create() -> *mut ::libc::c_void;
    pub fn wxStopWatch_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStopWatch_Start(_obj: *mut ::libc::c_void, msec: ::libc::c_int)
     -> ();
    pub fn wxStopWatch_Pause(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStopWatch_Resume(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStopWatch_Time(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxStreamBase_GetLastError(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStreamBase_GetSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxStreamBase_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSystemSettings_GetColour(index: ::libc::c_int,
                                      _ref: *mut ::libc::c_void) -> ();
    pub fn wxSystemSettings_GetFont(index: ::libc::c_int,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxSystemSettings_GetMetric(index: ::libc::c_int) -> ::libc::c_int;
    pub fn wxSystemSettings_GetScreenType() -> ::libc::c_int;
    pub fn wxTaskBarIcon_Create() -> *mut ::libc::c_void;
    pub fn wxTaskBarIcon_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTaskBarIcon_IsIconInstalled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTaskBarIcon_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTaskBarIcon_PopupMenu(_obj: *mut ::libc::c_void,
                                   menu: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTaskBarIcon_RemoveIcon(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTaskBarIcon_SetIcon(_obj: *mut ::libc::c_void,
                                 icon: *mut ::libc::c_void,
                                 text: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_AppendText(_obj: *mut ::libc::c_void,
                                 text: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_CanCopy(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_CanCut(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_CanPaste(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_CanRedo(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_CanUndo(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_ChangeValue(_obj: *mut ::libc::c_void,
                                  text: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_Copy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _txt: *mut ::libc::c_void, _lft: ::libc::c_int,
                             _top: ::libc::c_int, _wdt: ::libc::c_int,
                             _hgt: ::libc::c_int, _stl: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxTextCtrl_Cut(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_DiscardEdits(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_GetInsertionPoint(_obj: *mut ::libc::c_void)
     -> ::libc::c_long;
    pub fn wxTextCtrl_GetLastPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_long;
    pub fn wxTextCtrl_GetLineLength(_obj: *mut ::libc::c_void,
                                    lineNo: ::libc::c_long) -> ::libc::c_int;
    pub fn wxTextCtrl_GetLineText(_obj: *mut ::libc::c_void,
                                  lineNo: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxTextCtrl_GetNumberOfLines(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextCtrl_GetSelection(_obj: *mut ::libc::c_void,
                                   from: *mut ::libc::c_void,
                                   to: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_GetValue(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextCtrl_IsEditable(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_IsModified(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_LoadFile(_obj: *mut ::libc::c_void,
                               file: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_Paste(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_PositionToXY(_obj: *mut ::libc::c_void,
                                   pos: ::libc::c_long,
                                   x: *mut ::libc::c_long,
                                   y: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn wxTextCtrl_Redo(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_Remove(_obj: *mut ::libc::c_void, from: ::libc::c_long,
                             to: ::libc::c_long) -> ();
    pub fn wxTextCtrl_Replace(_obj: *mut ::libc::c_void, from: ::libc::c_long,
                              to: ::libc::c_long, value: *mut ::libc::c_void)
     -> ();
    pub fn wxTextCtrl_SaveFile(_obj: *mut ::libc::c_void,
                               file: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_SetEditable(_obj: *mut ::libc::c_void,
                                  editable: ::libc::c_int) -> ();
    pub fn wxTextCtrl_SetInsertionPoint(_obj: *mut ::libc::c_void,
                                        pos: ::libc::c_long) -> ();
    pub fn wxTextCtrl_SetInsertionPointEnd(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_SetSelection(_obj: *mut ::libc::c_void,
                                   from: ::libc::c_long, to: ::libc::c_long)
     -> ();
    pub fn wxTextCtrl_SetValue(_obj: *mut ::libc::c_void,
                               value: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_ShowPosition(_obj: *mut ::libc::c_void,
                                   pos: ::libc::c_long) -> ();
    pub fn wxTextCtrl_Undo(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_WriteText(_obj: *mut ::libc::c_void,
                                text: *mut ::libc::c_void) -> ();
    pub fn wxTextCtrl_XYToPosition(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_long, y: ::libc::c_long)
     -> ::libc::c_long;
    pub fn TextDataObject_Create(_txt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn TextDataObject_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn TextDataObject_GetTextLength(_obj: *mut ::libc::c_void) -> size_t;
    pub fn TextDataObject_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn TextDataObject_SetText(_obj: *mut ::libc::c_void,
                                  text: *mut ::libc::c_void) -> ();
    pub fn wxTextValidator_Create(style: ::libc::c_int,
                                  val: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextValidator_GetExcludes(_obj: *mut ::libc::c_void,
                                       _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextValidator_GetIncludes(_obj: *mut ::libc::c_void,
                                       _ref: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextValidator_SetExcludes(_obj: *mut ::libc::c_void,
                                       list: *mut ::libc::c_void,
                                       count: ::libc::c_int) -> ();
    pub fn wxTextValidator_SetIncludes(_obj: *mut ::libc::c_void,
                                       list: *mut ::libc::c_void,
                                       count: ::libc::c_int) -> ();
    pub fn wxTextValidator_Clone(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextValidator_TransferToWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextValidator_TransferFromWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextValidator_GetStyle(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextValidator_OnChar(_obj: *mut ::libc::c_void,
                                  event: *mut ::libc::c_void) -> ();
    pub fn wxTextValidator_SetStyle(_obj: *mut ::libc::c_void,
                                    style: ::libc::c_int) -> ();
    pub fn wxThinSplitterWindow_Create(parent: *mut ::libc::c_void,
                                       id: ::libc::c_int, x: ::libc::c_int,
                                       y: ::libc::c_int, w: ::libc::c_int,
                                       h: ::libc::c_int, style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxThinSplitterWindow_DrawSash(_obj: *mut ::libc::c_void,
                                         dc: *mut ::libc::c_void) -> ();
    pub fn wxThinSplitterWindow_SashHitTest(_obj: *mut ::libc::c_void,
                                            x: ::libc::c_int,
                                            y: ::libc::c_int,
                                            tolerance: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxThinSplitterWindow_SizeWindows(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTimer_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxTimer_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTimer_GetInterval(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTimer_IsOneShot(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTimer_IsRuning(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTimer_Start(_obj: *mut ::libc::c_void, _int: ::libc::c_int,
                         _one: ::libc::c_int) -> ::libc::c_int;
    pub fn wxTimer_Stop(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTimerEvent_GetInterval(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTipWindow_Close(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTipWindow_Create(parent: *mut ::libc::c_void,
                              text: *mut ::libc::c_void,
                              maxLength: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxTipWindow_SetBoundingRect(_obj: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int,
                                       w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxTipWindow_SetTipWindowPtr(_obj: *mut ::libc::c_void,
                                       windowPtr: *mut ::libc::c_void) -> ();
    pub fn wxToggleButton_Create(parent: *mut ::libc::c_void,
                                 id: ::libc::c_int,
                                 label: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int, w: ::libc::c_int,
                                 h: ::libc::c_int, style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxToggleButton_Enable(_obj: *mut ::libc::c_void,
                                 enable: ::libc::c_int) -> ::libc::c_int;
    pub fn wxToggleButton_GetValue(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxToggleButton_SetLabel(_obj: *mut ::libc::c_void,
                                   label: *mut ::libc::c_void) -> ();
    pub fn wxToggleButton_SetValue(_obj: *mut ::libc::c_void,
                                   state: ::libc::c_int) -> ();
    pub fn wxToolBar_AddControl(_obj: *mut ::libc::c_void,
                                ctrl: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxToolBar_AddSeparator(_obj: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_AddTool(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                             bmp: *mut ::libc::c_void,
                             shelp: *mut ::libc::c_void,
                             lhelp: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_AddToolEx(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                               bmp1: *mut ::libc::c_void,
                               bmp2: *mut ::libc::c_void,
                               isToggle: ::libc::c_int, x: ::libc::c_int,
                               y: ::libc::c_int, data: *mut ::libc::c_void,
                               shelp: *mut ::libc::c_void,
                               lhelp: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                            _lft: ::libc::c_int, _top: ::libc::c_int,
                            _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                            _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxToolBar_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_DeleteTool(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxToolBar_DeleteToolByPos(_obj: *mut ::libc::c_void,
                                     pos: ::libc::c_int) -> ::libc::c_int;
    pub fn wxToolBar_EnableTool(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                                enable: ::libc::c_int) -> ();
    pub fn wxToolBar_GetMargins(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_GetToolBitmapSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_GetToolClientData(_obj: *mut ::libc::c_void,
                                       id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_GetToolEnabled(_obj: *mut ::libc::c_void,
                                    id: ::libc::c_int) -> ::libc::c_int;
    pub fn wxToolBar_GetToolLongHelp(_obj: *mut ::libc::c_void,
                                     id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_GetToolPacking(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxToolBar_GetToolShortHelp(_obj: *mut ::libc::c_void,
                                      id: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_GetToolSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_GetToolState(_obj: *mut ::libc::c_void,
                                  id: ::libc::c_int) -> ::libc::c_int;
    pub fn wxToolBar_InsertControl(_obj: *mut ::libc::c_void,
                                   pos: ::libc::c_int,
                                   ctrl: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_InsertSeparator(_obj: *mut ::libc::c_void,
                                     pos: ::libc::c_int) -> ();
    pub fn wxToolBar_InsertTool(_obj: *mut ::libc::c_void, pos: ::libc::c_int,
                                id: ::libc::c_int, bmp1: *mut ::libc::c_void,
                                bmp2: *mut ::libc::c_void,
                                isToggle: ::libc::c_int,
                                data: *mut ::libc::c_void,
                                shelp: *mut ::libc::c_void,
                                lhelp: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_Realize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxToolBar_RemoveTool(_obj: *mut ::libc::c_void, id: ::libc::c_int)
     -> ();
    pub fn wxToolBar_SetMargins(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int) -> ();
    pub fn wxToolBar_SetToolBitmapSize(_obj: *mut ::libc::c_void,
                                       x: ::libc::c_int, y: ::libc::c_int)
     -> ();
    pub fn wxToolBar_SetToolClientData(_obj: *mut ::libc::c_void,
                                       id: ::libc::c_int,
                                       data: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_SetToolLongHelp(_obj: *mut ::libc::c_void,
                                     id: ::libc::c_int,
                                     str: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_SetToolPacking(_obj: *mut ::libc::c_void,
                                    packing: ::libc::c_int) -> ();
    pub fn wxToolBar_SetToolSeparation(_obj: *mut ::libc::c_void,
                                       separation: ::libc::c_int) -> ();
    pub fn wxToolBar_SetToolShortHelp(_obj: *mut ::libc::c_void,
                                      id: ::libc::c_int,
                                      str: *mut ::libc::c_void) -> ();
    pub fn wxToolBar_ToggleTool(_obj: *mut ::libc::c_void, id: ::libc::c_int,
                                toggle: ::libc::c_int) -> ();
    pub fn wxToolLayoutItem_IsSeparator(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxToolLayoutItem_Rect(_obj: *mut ::libc::c_void,
                                 _x: *mut ::libc::c_void,
                                 _y: *mut ::libc::c_void,
                                 _w: *mut ::libc::c_void,
                                 _h: *mut ::libc::c_void) -> ();
    pub fn wxToolWindow_AddMiniButton(_obj: *mut ::libc::c_void,
                                      _btn: *mut ::libc::c_void) -> ();
    pub fn wxToolWindow_Create(_obj: *mut ::libc::c_void,
                               _btn: *mut ::libc::c_void,
                               _ttl: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxToolWindow_GetClient(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxToolWindow_SetClient(_obj: *mut ::libc::c_void,
                                  _wnd: *mut ::libc::c_void) -> ();
    pub fn wxToolWindow_SetTitleFont(_obj: *mut ::libc::c_void,
                                     _fnt: *mut ::libc::c_void) -> ();
    pub fn wxTopLevelWindow_EnableCloseButton(_obj: *mut ::libc::c_void,
                                              enable: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxTopLevelWindow_GetDefaultButton(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTopLevelWindow_GetDefaultItem(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTopLevelWindow_GetIcon(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTopLevelWindow_GetTitle(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTopLevelWindow_Iconize(_obj: *mut ::libc::c_void,
                                    iconize: ::libc::c_int) -> ::libc::c_int;
    pub fn wxTopLevelWindow_IsActive(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTopLevelWindow_IsIconized(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTopLevelWindow_IsMaximized(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTopLevelWindow_Maximize(_obj: *mut ::libc::c_void,
                                     maximize: ::libc::c_int) -> ();
    pub fn wxTopLevelWindow_RequestUserAttention(_obj: *mut ::libc::c_void,
                                                 flags: ::libc::c_int) -> ();
    pub fn wxTopLevelWindow_SetDefaultButton(_obj: *mut ::libc::c_void,
                                             pBut: *mut ::libc::c_void) -> ();
    pub fn wxTopLevelWindow_SetDefaultItem(_obj: *mut ::libc::c_void,
                                           pBut: *mut ::libc::c_void) -> ();
    pub fn wxTopLevelWindow_SetIcon(_obj: *mut ::libc::c_void,
                                    pIcon: *mut ::libc::c_void) -> ();
    pub fn wxTopLevelWindow_SetIcons(_obj: *mut ::libc::c_void,
                                     _icons: *mut ::libc::c_void) -> ();
    pub fn wxTopLevelWindow_SetMaxSize(_obj: *mut ::libc::c_void,
                                       w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxTopLevelWindow_SetMinSize(_obj: *mut ::libc::c_void,
                                       w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxTopLevelWindow_SetTitle(_obj: *mut ::libc::c_void,
                                     pString: *mut ::libc::c_void) -> ();
    pub fn wxTreeCompanionWindow_Create(parent: *mut ::libc::c_void,
                                        id: ::libc::c_int, x: ::libc::c_int,
                                        y: ::libc::c_int, w: ::libc::c_int,
                                        h: ::libc::c_int,
                                        style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxTreeCompanionWindow_DrawItem(_obj: *mut ::libc::c_void,
                                          dc: *mut ::libc::c_void,
                                          id: *mut ::libc::c_void,
                                          x: ::libc::c_int, y: ::libc::c_int,
                                          w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxTreeCompanionWindow_GetTreeCtrl(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCompanionWindow_SetTreeCtrl(_obj: *mut ::libc::c_void,
                                             treeCtrl: *mut ::libc::c_void)
     -> ();
    pub fn wxTreeCtrl_AddRoot(_obj: *mut ::libc::c_void,
                              text: *mut ::libc::c_void, image: ::libc::c_int,
                              selectedImage: ::libc::c_int,
                              data: *mut ::libc::c_void,
                              _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_AppendItem(_obj: *mut ::libc::c_void,
                                 parent: *mut ::libc::c_void,
                                 text: *mut ::libc::c_void,
                                 image: ::libc::c_int,
                                 selectedImage: ::libc::c_int,
                                 data: *mut ::libc::c_void,
                                 _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_Collapse(_obj: *mut ::libc::c_void,
                               item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_CollapseAndReset(_obj: *mut ::libc::c_void,
                                       item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_Create(_obj: *mut ::libc::c_void,
                             _cmp: *mut ::libc::c_void,
                             _prt: *mut ::libc::c_void, _id: ::libc::c_int,
                             _lft: ::libc::c_int, _top: ::libc::c_int,
                             _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                             _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_Delete(_obj: *mut ::libc::c_void,
                             item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_DeleteAllItems(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_DeleteChildren(_obj: *mut ::libc::c_void,
                                     item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_EditLabel(_obj: *mut ::libc::c_void,
                                item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_EndEditLabel(_obj: *mut ::libc::c_void,
                                   item: *mut ::libc::c_void,
                                   discardChanges: ::libc::c_int) -> ();
    pub fn wxTreeCtrl_EnsureVisible(_obj: *mut ::libc::c_void,
                                    item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_Expand(_obj: *mut ::libc::c_void,
                             item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetBoundingRect(_obj: *mut ::libc::c_void,
                                      item: *mut ::libc::c_void,
                                      textOnly: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_GetChildrenCount(_obj: *mut ::libc::c_void,
                                       item: *mut ::libc::c_void,
                                       recursively: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxTreeCtrl_GetCount(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_GetEditControl(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_GetFirstChild(_obj: *mut ::libc::c_void,
                                    item: *mut ::libc::c_void,
                                    cookie: *mut ::libc::c_int,
                                    _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetFirstVisibleItem(_obj: *mut ::libc::c_void,
                                          item: *mut ::libc::c_void,
                                          _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetImageList(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_GetIndent(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_GetItemData(_obj: *mut ::libc::c_void,
                                  item: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_GetItemImage(_obj: *mut ::libc::c_void,
                                   item: *mut ::libc::c_void,
                                   which: ::libc::c_int) -> ::libc::c_int;
    pub fn wxTreeCtrl_GetItemText(_obj: *mut ::libc::c_void,
                                  item: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_GetLastChild(_obj: *mut ::libc::c_void,
                                   item: *mut ::libc::c_void,
                                   _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetNextChild(_obj: *mut ::libc::c_void,
                                   item: *mut ::libc::c_void,
                                   cookie: *mut ::libc::c_int,
                                   _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetNextSibling(_obj: *mut ::libc::c_void,
                                     item: *mut ::libc::c_void,
                                     _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetNextVisible(_obj: *mut ::libc::c_void,
                                     item: *mut ::libc::c_void,
                                     _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetParent(_obj: *mut ::libc::c_void,
                                item: *mut ::libc::c_void,
                                _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetPrevSibling(_obj: *mut ::libc::c_void,
                                     item: *mut ::libc::c_void,
                                     _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetPrevVisible(_obj: *mut ::libc::c_void,
                                     item: *mut ::libc::c_void,
                                     _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetRootItem(_obj: *mut ::libc::c_void,
                                  _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetSelection(_obj: *mut ::libc::c_void,
                                   _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetSelections(_obj: *mut ::libc::c_void,
                                    selections: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTreeCtrl_GetSpacing(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_GetStateImageList(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_HitTest(_obj: *mut ::libc::c_void, _x: ::libc::c_int,
                              _y: ::libc::c_int, flags: *mut ::libc::c_int,
                              _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_InsertItem(_obj: *mut ::libc::c_void,
                                 parent: *mut ::libc::c_void,
                                 idPrevious: *mut ::libc::c_void,
                                 text: *mut ::libc::c_void,
                                 image: ::libc::c_int,
                                 selectedImage: ::libc::c_int,
                                 data: *mut ::libc::c_void,
                                 _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_InsertItemByIndex(_obj: *mut ::libc::c_void,
                                        parent: *mut ::libc::c_void,
                                        index: ::libc::c_int,
                                        text: *mut ::libc::c_void,
                                        image: ::libc::c_int,
                                        selectedImage: ::libc::c_int,
                                        data: *mut ::libc::c_void,
                                        _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_IsBold(_obj: *mut ::libc::c_void,
                             item: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_IsExpanded(_obj: *mut ::libc::c_void,
                                 item: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_IsSelected(_obj: *mut ::libc::c_void,
                                 item: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_IsVisible(_obj: *mut ::libc::c_void,
                                item: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeCtrl_ItemHasChildren(_obj: *mut ::libc::c_void,
                                      item: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTreeCtrl_OnCompareItems(_obj: *mut ::libc::c_void,
                                     item1: *mut ::libc::c_void,
                                     item2: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTreeCtrl_PrependItem(_obj: *mut ::libc::c_void,
                                  parent: *mut ::libc::c_void,
                                  text: *mut ::libc::c_void,
                                  image: ::libc::c_int,
                                  selectedImage: ::libc::c_int,
                                  data: *mut ::libc::c_void,
                                  _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_ScrollTo(_obj: *mut ::libc::c_void,
                               item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SelectItem(_obj: *mut ::libc::c_void,
                                 item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetImageList(_obj: *mut ::libc::c_void,
                                   imageList: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetIndent(_obj: *mut ::libc::c_void,
                                indent: ::libc::c_int) -> ();
    pub fn wxTreeCtrl_SetItemBackgroundColour(_obj: *mut ::libc::c_void,
                                              item: *mut ::libc::c_void,
                                              col: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetItemBold(_obj: *mut ::libc::c_void,
                                  item: *mut ::libc::c_void,
                                  bold: ::libc::c_int) -> ();
    pub fn wxTreeCtrl_SetItemData(_obj: *mut ::libc::c_void,
                                  item: *mut ::libc::c_void,
                                  data: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetItemDropHighlight(_obj: *mut ::libc::c_void,
                                           item: *mut ::libc::c_void,
                                           highlight: ::libc::c_int) -> ();
    pub fn wxTreeCtrl_SetItemFont(_obj: *mut ::libc::c_void,
                                  item: *mut ::libc::c_void,
                                  font: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetItemHasChildren(_obj: *mut ::libc::c_void,
                                         item: *mut ::libc::c_void,
                                         hasChildren: ::libc::c_int) -> ();
    pub fn wxTreeCtrl_SetItemImage(_obj: *mut ::libc::c_void,
                                   item: *mut ::libc::c_void,
                                   image: ::libc::c_int, which: ::libc::c_int)
     -> ();
    pub fn wxTreeCtrl_SetItemText(_obj: *mut ::libc::c_void,
                                  item: *mut ::libc::c_void,
                                  text: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetItemTextColour(_obj: *mut ::libc::c_void,
                                        item: *mut ::libc::c_void,
                                        col: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SetSpacing(_obj: *mut ::libc::c_void,
                                 spacing: ::libc::c_int) -> ();
    pub fn wxTreeCtrl_SetStateImageList(_obj: *mut ::libc::c_void,
                                        imageList: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_SortChildren(_obj: *mut ::libc::c_void,
                                   item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_Toggle(_obj: *mut ::libc::c_void,
                             item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_Unselect(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_UnselectAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTreeEvent_GetCode(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTreeEvent_GetItem(_obj: *mut ::libc::c_void,
                               _ref: *mut ::libc::c_void) -> ();
    pub fn wxTreeEvent_GetLabel(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeEvent_GetOldItem(_obj: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxTreeEvent_GetPoint(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeItemId_Create() -> *mut ::libc::c_void;
    pub fn wxTreeItemId_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTreeItemId_IsOk(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxUpdateUIEvent_Check(_obj: *mut ::libc::c_void,
                                 check: ::libc::c_int) -> ();
    pub fn wxUpdateUIEvent_CopyObject(_obj: *mut ::libc::c_void,
                                      obj: *mut ::libc::c_void) -> ();
    pub fn wxUpdateUIEvent_Enable(_obj: *mut ::libc::c_void,
                                  enable: ::libc::c_int) -> ();
    pub fn wxUpdateUIEvent_GetChecked(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxUpdateUIEvent_GetEnabled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxUpdateUIEvent_GetSetChecked(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxUpdateUIEvent_GetSetEnabled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxUpdateUIEvent_GetSetText(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxUpdateUIEvent_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxUpdateUIEvent_SetText(_obj: *mut ::libc::c_void,
                                   text: *mut ::libc::c_void) -> ();
    pub fn wxValidator_Create() -> *mut ::libc::c_void;
    pub fn wxValidator_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxValidator_GetWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxValidator_SetBellOnError(doIt: ::libc::c_int) -> ();
    pub fn wxValidator_SetWindow(_obj: *mut ::libc::c_void,
                                 win: *mut ::libc::c_void) -> ();
    pub fn wxValidator_TransferFromWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxValidator_TransferToWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxValidator_Validate(_obj: *mut ::libc::c_void,
                                parent: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_AddChild(_obj: *mut ::libc::c_void,
                             child: *mut ::libc::c_void) -> ();
    pub fn wxWindow_AddConstraintReference(_obj: *mut ::libc::c_void,
                                           otherWin: *mut ::libc::c_void)
     -> ();
    pub fn wxWindow_CaptureMouse(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_Center(_obj: *mut ::libc::c_void,
                           direction: ::libc::c_int) -> ();
    pub fn wxWindow_CenterOnParent(_obj: *mut ::libc::c_void,
                                   dir: ::libc::c_int) -> ();
    pub fn wxWindow_ClearBackground(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_ClientToScreen(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxWindow_Close(_obj: *mut ::libc::c_void, _force: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxWindow_ConvertDialogToPixels(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_ConvertPixelsToDialog(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                           _x: ::libc::c_int, _y: ::libc::c_int,
                           _w: ::libc::c_int, _h: ::libc::c_int,
                           _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxWindow_DeleteRelatedConstraints(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_Destroy(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_DestroyChildren(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_Disable(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_DoPhase(_obj: *mut ::libc::c_void, phase: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxWindow_Enable(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_FindFocus(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_FindWindow(_obj: *mut ::libc::c_void,
                               name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_Fit(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_FitInside(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_Freeze(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_GetEffectiveMinSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetAutoLayout(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetBackgroundColour(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxWindow_GetBestSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetCaret(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetCharHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetCharWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetChildren(_obj: *mut ::libc::c_void,
                                _res: *mut ::libc::c_void,
                                _cnt: ::libc::c_int) -> ::libc::c_int;
    pub fn wxWindow_GetClientData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetClientSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetClientSizeConstraint(_obj: *mut ::libc::c_void,
                                            _w: *mut ::libc::c_int,
                                            _h: *mut ::libc::c_int) -> ();
    pub fn wxWindow_GetConstraints(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetConstraintsInvolvedIn(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetCursor(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetDropTarget(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetEventHandler(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetFont(_obj: *mut ::libc::c_void,
                            _ref: *mut ::libc::c_void) -> ();
    pub fn wxWindow_GetForegroundColour(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxWindow_GetHandle(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetId(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetLabel(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetLabelEmpty(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetMaxHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetMaxWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetMinHeight(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetMinWidth(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_GetName(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxWindow_GetParent(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetPositionConstraint(_obj: *mut ::libc::c_void,
                                          _x: *mut ::libc::c_int,
                                          _y: *mut ::libc::c_int) -> ();
    pub fn wxWindow_GetRect(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxWindow_GetScrollPos(_obj: *mut ::libc::c_void,
                                 orient: ::libc::c_int) -> ::libc::c_int;
    pub fn wxWindow_GetScrollRange(_obj: *mut ::libc::c_void,
                                   orient: ::libc::c_int) -> ::libc::c_int;
    pub fn wxWindow_GetScrollThumb(_obj: *mut ::libc::c_void,
                                   orient: ::libc::c_int) -> ::libc::c_int;
    pub fn wxWindow_GetSize(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxWindow_GetSizeConstraint(_obj: *mut ::libc::c_void,
                                      _w: *mut ::libc::c_int,
                                      _h: *mut ::libc::c_int) -> ();
    pub fn wxWindow_GetSizer(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetTextExtent(_obj: *mut ::libc::c_void,
                                  string: *mut ::libc::c_void,
                                  x: *mut ::libc::c_int,
                                  y: *mut ::libc::c_int,
                                  descent: *mut ::libc::c_int,
                                  externalLeading: *mut ::libc::c_int,
                                  theFont: *mut ::libc::c_void) -> ();
    pub fn wxWindow_GetToolTip(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetUpdateRegion(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetValidator(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetVirtualSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_GetWindowStyleFlag(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_HasFlag(_obj: *mut ::libc::c_void, flag: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxWindow_Hide(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_InitDialog(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_IsBeingDeleted(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_IsEnabled(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_IsExposed(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, w: ::libc::c_int,
                              h: ::libc::c_int) -> ::libc::c_int;
    pub fn wxWindow_IsShown(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_IsTopLevel(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_Layout(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_LayoutPhase1(_obj: *mut ::libc::c_void,
                                 noChanges: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxWindow_LayoutPhase2(_obj: *mut ::libc::c_void,
                                 noChanges: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxWindow_Lower(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_MakeModal(_obj: *mut ::libc::c_void, modal: ::libc::c_int)
     -> ();
    pub fn wxWindow_Move(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                         y: ::libc::c_int) -> ();
    pub fn wxWindow_MoveConstraint(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxWindow_PopEventHandler(_obj: *mut ::libc::c_void,
                                    deleteHandler: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxWindow_PopupMenu(_obj: *mut ::libc::c_void,
                              menu: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int) -> ::libc::c_int;
    pub fn wxWindow_PrepareDC(_obj: *mut ::libc::c_void,
                              dc: *mut ::libc::c_void) -> ();
    pub fn wxWindow_PushEventHandler(_obj: *mut ::libc::c_void,
                                     handler: *mut ::libc::c_void) -> ();
    pub fn wxWindow_Raise(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_Refresh(_obj: *mut ::libc::c_void,
                            eraseBackground: ::libc::c_int) -> ();
    pub fn wxWindow_RefreshRect(_obj: *mut ::libc::c_void,
                                eraseBackground: ::libc::c_int,
                                x: ::libc::c_int, y: ::libc::c_int,
                                w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxWindow_ReleaseMouse(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_RemoveChild(_obj: *mut ::libc::c_void,
                                child: *mut ::libc::c_void) -> ();
    pub fn wxWindow_RemoveConstraintReference(_obj: *mut ::libc::c_void,
                                              otherWin: *mut ::libc::c_void)
     -> ();
    pub fn wxWindow_Reparent(_obj: *mut ::libc::c_void,
                             _par: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_ResetConstraints(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_ScreenToClient(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxWindow_ScrollWindow(_obj: *mut ::libc::c_void, dx: ::libc::c_int,
                                 dy: ::libc::c_int) -> ();
    pub fn wxWindow_ScrollWindowRect(_obj: *mut ::libc::c_void,
                                     dx: ::libc::c_int, dy: ::libc::c_int,
                                     x: ::libc::c_int, y: ::libc::c_int,
                                     w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetAcceleratorTable(_obj: *mut ::libc::c_void,
                                        accel: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetAutoLayout(_obj: *mut ::libc::c_void,
                                  autoLayout: ::libc::c_int) -> ();
    pub fn wxWindow_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                        colour: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_SetCaret(_obj: *mut ::libc::c_void,
                             caret: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetClientData(_obj: *mut ::libc::c_void,
                                  data: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetClientObject(_obj: *mut ::libc::c_void,
                                    data: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetClientSize(_obj: *mut ::libc::c_void,
                                  width: ::libc::c_int, height: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetConstraintSizes(_obj: *mut ::libc::c_void,
                                       recurse: ::libc::c_int) -> ();
    pub fn wxWindow_SetConstraints(_obj: *mut ::libc::c_void,
                                   constraints: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetCursor(_obj: *mut ::libc::c_void,
                              cursor: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_SetDropTarget(_obj: *mut ::libc::c_void,
                                  dropTarget: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetExtraStyle(_obj: *mut ::libc::c_void,
                                  exStyle: ::libc::c_long) -> ();
    pub fn wxWindow_SetFocus(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetFont(_obj: *mut ::libc::c_void,
                            font: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_SetForegroundColour(_obj: *mut ::libc::c_void,
                                        colour: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_SetId(_obj: *mut ::libc::c_void, _id: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetLabel(_obj: *mut ::libc::c_void,
                             _title: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetName(_obj: *mut ::libc::c_void,
                            _name: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetScrollPos(_obj: *mut ::libc::c_void,
                                 orient: ::libc::c_int, pos: ::libc::c_int,
                                 refresh: ::libc::c_int) -> ();
    pub fn wxWindow_SetScrollbar(_obj: *mut ::libc::c_void,
                                 orient: ::libc::c_int, pos: ::libc::c_int,
                                 thumbVisible: ::libc::c_int,
                                 range: ::libc::c_int, refresh: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetSize(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                            y: ::libc::c_int, width: ::libc::c_int,
                            height: ::libc::c_int, sizeFlags: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetSizeConstraint(_obj: *mut ::libc::c_void,
                                      x: ::libc::c_int, y: ::libc::c_int,
                                      w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetSizeHints(_obj: *mut ::libc::c_void,
                                 minW: ::libc::c_int, minH: ::libc::c_int,
                                 maxW: ::libc::c_int, maxH: ::libc::c_int,
                                 incW: ::libc::c_int, incH: ::libc::c_int)
     -> ();
    pub fn wxWindow_SetSizer(_obj: *mut ::libc::c_void,
                             sizer: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetToolTip(_obj: *mut ::libc::c_void,
                               tip: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetValidator(_obj: *mut ::libc::c_void,
                                 validator: *mut ::libc::c_void) -> ();
    pub fn wxWindow_SetWindowStyleFlag(_obj: *mut ::libc::c_void,
                                       style: ::libc::c_long) -> ();
    pub fn wxWindow_Show(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_Thaw(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_TransferDataFromWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_TransferDataToWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWindow_UnsetConstraints(_obj: *mut ::libc::c_void,
                                     c: *mut ::libc::c_void) -> ();
    pub fn wxWindow_UpdateWindowUI(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindow_Validate(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxWindow_SetVirtualSize(_obj: *mut ::libc::c_void,
                                   w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxWindow_WarpPointer(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                                y: ::libc::c_int) -> ();
    pub fn wxWindowCreateEvent_GetWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindowDC_Create(win: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxWindowDC_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxWindowDestroyEvent_GetWindow(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWizard_Chain(f: *mut ::libc::c_void, s: *mut ::libc::c_void)
     -> ();
    pub fn wxWizard_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                           _txt: *mut ::libc::c_void,
                           _bmp: *mut ::libc::c_void, _lft: ::libc::c_int,
                           _top: ::libc::c_int, _wdt: ::libc::c_int,
                           _hgt: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxWizard_GetCurrentPage(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWizard_GetPageSize(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWizard_RunWizard(_obj: *mut ::libc::c_void,
                              firstPage: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWizard_SetPageSize(_obj: *mut ::libc::c_void, w: ::libc::c_int,
                                h: ::libc::c_int) -> ();
    pub fn wxWizardEvent_GetDirection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxWizardPageSimple_Create(_prt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWizardPageSimple_GetBitmap(_obj: *mut ::libc::c_void,
                                        _ref: *mut ::libc::c_void) -> ();
    pub fn wxWizardPageSimple_GetNext(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWizardPageSimple_GetPrev(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWizardPageSimple_SetNext(_obj: *mut ::libc::c_void,
                                      next: *mut ::libc::c_void) -> ();
    pub fn wxWizardPageSimple_SetPrev(_obj: *mut ::libc::c_void,
                                      prev: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_AddHandler(_obj: *mut ::libc::c_void,
                                    handler: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_AddSubclassFactory(_obj: *mut ::libc::c_void,
                                            factory: *mut ::libc::c_void)
     -> ();
    pub fn wxXmlResource_AttachUnknownControl(_obj: *mut ::libc::c_void,
                                              control: *mut ::libc::c_void,
                                              parent: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxXmlResource_ClearHandlers(_obj: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_CompareVersion(_obj: *mut ::libc::c_void,
                                        major: ::libc::c_int,
                                        minor: ::libc::c_int,
                                        release: ::libc::c_int,
                                        revision: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxXmlResource_Create(flags: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxXmlResource_CreateFromFile(filemask: *mut ::libc::c_void,
                                        flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_Get() -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetDomain(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetFlags(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxXmlResource_GetVersion(_obj: *mut ::libc::c_void)
     -> ::libc::c_long;
    pub fn wxXmlResource_GetXRCID(_obj: *mut ::libc::c_void,
                                  str_id: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxXmlResource_InitAllHandlers(_obj: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_InsertHandler(_obj: *mut ::libc::c_void,
                                       handler: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_Load(_obj: *mut ::libc::c_void,
                              filemask: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxXmlResource_LoadBitmap(_obj: *mut ::libc::c_void,
                                    name: *mut ::libc::c_void,
                                    _ref: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_LoadDialog(_obj: *mut ::libc::c_void,
                                    parent: *mut ::libc::c_void,
                                    name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_LoadFrame(_obj: *mut ::libc::c_void,
                                   parent: *mut ::libc::c_void,
                                   name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_LoadIcon(_obj: *mut ::libc::c_void,
                                  name: *mut ::libc::c_void,
                                  _ref: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_LoadMenu(_obj: *mut ::libc::c_void,
                                  name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_LoadMenuBar(_obj: *mut ::libc::c_void,
                                     parent: *mut ::libc::c_void,
                                     name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_LoadPanel(_obj: *mut ::libc::c_void,
                                   parent: *mut ::libc::c_void,
                                   name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_LoadToolBar(_obj: *mut ::libc::c_void,
                                     parent: *mut ::libc::c_void,
                                     name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetSizer(_obj: *mut ::libc::c_void,
                                  str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetBoxSizer(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetStaticBoxSizer(_obj: *mut ::libc::c_void,
                                           str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetGridSizer(_obj: *mut ::libc::c_void,
                                      str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetFlexGridSizer(_obj: *mut ::libc::c_void,
                                          str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetBitmapButton(_obj: *mut ::libc::c_void,
                                         str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetButton(_obj: *mut ::libc::c_void,
                                   str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetCalendarCtrl(_obj: *mut ::libc::c_void,
                                         str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetCheckBox(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetCheckListBox(_obj: *mut ::libc::c_void,
                                         str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetChoice(_obj: *mut ::libc::c_void,
                                   str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetComboBox(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetGauge(_obj: *mut ::libc::c_void,
                                  str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetGrid(_obj: *mut ::libc::c_void,
                                 str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetHtmlWindow(_obj: *mut ::libc::c_void,
                                       str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetListBox(_obj: *mut ::libc::c_void,
                                    str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetListCtrl(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetMDIChildFrame(_obj: *mut ::libc::c_void,
                                          str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetMDIParentFrame(_obj: *mut ::libc::c_void,
                                           str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetMenu(_obj: *mut ::libc::c_void,
                                 str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetMenuBar(_obj: *mut ::libc::c_void,
                                    str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetMenuItem(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetNotebook(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetPanel(_obj: *mut ::libc::c_void,
                                  str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetRadioButton(_obj: *mut ::libc::c_void,
                                        str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetRadioBox(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetScrollBar(_obj: *mut ::libc::c_void,
                                      str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetScrolledWindow(_obj: *mut ::libc::c_void,
                                           str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetSlider(_obj: *mut ::libc::c_void,
                                   str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetSpinButton(_obj: *mut ::libc::c_void,
                                       str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetSpinCtrl(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetSplitterWindow(_obj: *mut ::libc::c_void,
                                           str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetStaticBitmap(_obj: *mut ::libc::c_void,
                                         str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetStaticBox(_obj: *mut ::libc::c_void,
                                      str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetStaticLine(_obj: *mut ::libc::c_void,
                                       str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetStaticText(_obj: *mut ::libc::c_void,
                                       str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetTextCtrl(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_GetTreeCtrl(_obj: *mut ::libc::c_void,
                                     str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxXmlResource_Unload(_obj: *mut ::libc::c_void,
                                filemask: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxXmlResource_Set(_obj: *mut ::libc::c_void,
                             res: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxXmlResource_SetDomain(_obj: *mut ::libc::c_void,
                                   domain: *mut ::libc::c_void) -> ();
    pub fn wxXmlResource_SetFlags(_obj: *mut ::libc::c_void,
                                  flags: ::libc::c_int) -> ();
    pub fn wxPropertyGrid_Append(_obj: *mut ::libc::c_void,
                                 prop: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPropertyGrid_Create(_prt: *mut ::libc::c_void,
                                 _id: ::libc::c_int, _lft: ::libc::c_int,
                                 _top: ::libc::c_int, _wdt: ::libc::c_int,
                                 _hgt: ::libc::c_int, _stl: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxPropertyGrid_DisableProperty(_obj: *mut ::libc::c_void,
                                          propName: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPropertyGridEvent_HasProperty(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxPropertyGridEvent_GetProperty(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPGProperty_GetLabel(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPGProperty_GetName(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPGProperty_GetValueAsString(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPGProperty_GetValueType(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPGProperty_SetHelpString(_obj: *mut ::libc::c_void,
                                      helpString: *mut ::libc::c_void) -> ();
    pub fn wxStringProperty_Create(label: *mut ::libc::c_void,
                                   name: *mut ::libc::c_void,
                                   value: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxIntProperty_Create(label: *mut ::libc::c_void,
                                name: *mut ::libc::c_void,
                                value: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxBoolProperty_Create(label: *mut ::libc::c_void,
                                 name: *mut ::libc::c_void,
                                 value: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxFloatProperty_Create(label: *mut ::libc::c_void,
                                  name: *mut ::libc::c_void,
                                  value: ::libc::c_float)
     -> *mut ::libc::c_void;
    pub fn wxDateProperty_Create(label: *mut ::libc::c_void,
                                 name: *mut ::libc::c_void,
                                 value: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxFileProperty_Create(label: *mut ::libc::c_void,
                                 name: *mut ::libc::c_void,
                                 value: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPropertyCategory_Create(label: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxDragImage_Create(image: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxDragIcon(icon: *mut ::libc::c_void, x: ::libc::c_int,
                      y: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxDragString(test: *mut ::libc::c_void, x: ::libc::c_int,
                        y: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxDragTreeItem(treeCtrl: *mut ::libc::c_void,
                          id: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxDragListItem(treeCtrl: *mut ::libc::c_void, id: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxGenericDragImage_Create(cursor: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGenericDragIcon(icon: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGenericDragString(test: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGenericDragTreeItem(treeCtrl: *mut ::libc::c_void,
                                 id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGenericDragListItem(treeCtrl: *mut ::libc::c_void,
                                 id: ::libc::c_long) -> *mut ::libc::c_void;
    pub fn wxDragImage_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxDragImage_BeginDragFullScreen(_self: *mut ::libc::c_void,
                                           x_pos: ::libc::c_int,
                                           y_pos: ::libc::c_int,
                                           window: *mut ::libc::c_void,
                                           fullScreen: ::libc::c_int,
                                           rect: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxDragImage_BeginDrag(_self: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int,
                                 window: *mut ::libc::c_void,
                                 boundingWindow: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGenericDragImage_DoDrawImage(_self: *mut ::libc::c_void,
                                          dc: *mut ::libc::c_void,
                                          x: ::libc::c_int, y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxDragImage_EndDrag(_self: *mut ::libc::c_void) -> ();
    pub fn wxGenericDragImage_GetImageRect(_self: *mut ::libc::c_void,
                                           x_pos: ::libc::c_int,
                                           y_pos: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxDragImage_Hide(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxDragImage_Move(_self: *mut ::libc::c_void, x: ::libc::c_int,
                            y: ::libc::c_int) -> ::libc::c_int;
    pub fn wxDragImage_Show(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGenericDragImage_UpdateBackingFromWindow(_self:
                                                          *mut ::libc::c_void,
                                                      windowDC:
                                                          *mut ::libc::c_void,
                                                      destDC:
                                                          *mut ::libc::c_void,
                                                      x: ::libc::c_int,
                                                      y: ::libc::c_int,
                                                      w: ::libc::c_int,
                                                      h: ::libc::c_int,
                                                      xdest: ::libc::c_int,
                                                      ydest: ::libc::c_int,
                                                      width: ::libc::c_int,
                                                      height: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGraphicsBrush_Create() -> *mut ::libc::c_void;
    pub fn wxGraphicsBrush_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_Create(dc: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsContext_CreateFromWindow(window: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsContext_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_CreateFromNative(context: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsContext_CreateFromNativeWindow(window:
                                                        *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsContext_Clip(_self: *mut ::libc::c_void,
                                  region: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_ClipByRectangle(_self: *mut ::libc::c_void,
                                             x: ::libc::c_double,
                                             y: ::libc::c_double,
                                             w: ::libc::c_double,
                                             h: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_ResetClip(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_DrawBitmap(_self: *mut ::libc::c_void,
                                        bmp: *mut ::libc::c_void,
                                        x: ::libc::c_double,
                                        y: ::libc::c_double,
                                        w: ::libc::c_double,
                                        h: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_DrawEllipse(_self: *mut ::libc::c_void,
                                         x: ::libc::c_double,
                                         y: ::libc::c_double,
                                         w: ::libc::c_double,
                                         h: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_DrawIcon(_self: *mut ::libc::c_void,
                                      icon: *mut ::libc::c_void,
                                      x: ::libc::c_double,
                                      y: ::libc::c_double,
                                      w: ::libc::c_double,
                                      h: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_DrawLines(_self: *mut ::libc::c_void, n: size_t,
                                       x: *mut ::libc::c_void,
                                       y: *mut ::libc::c_void,
                                       style: ::libc::c_int) -> ();
    pub fn wxGraphicsContext_DrawPath(_self: *mut ::libc::c_void,
                                      path: *mut ::libc::c_void,
                                      style: ::libc::c_int) -> ();
    pub fn wxGraphicsContext_DrawRectangle(_self: *mut ::libc::c_void,
                                           x: ::libc::c_double,
                                           y: ::libc::c_double,
                                           w: ::libc::c_double,
                                           h: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_DrawRoundedRectangle(_self: *mut ::libc::c_void,
                                                  x: ::libc::c_double,
                                                  y: ::libc::c_double,
                                                  w: ::libc::c_double,
                                                  h: ::libc::c_double,
                                                  radius: ::libc::c_double)
     -> ();
    pub fn wxGraphicsContext_DrawText(_self: *mut ::libc::c_void,
                                      text: *mut ::libc::c_void,
                                      x: ::libc::c_double,
                                      y: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_DrawTextWithAngle(_self: *mut ::libc::c_void,
                                               text: *mut ::libc::c_void,
                                               x: ::libc::c_double,
                                               y: ::libc::c_double,
                                               radius: ::libc::c_double)
     -> ();
    pub fn wxGraphicsContext_FillPath(_self: *mut ::libc::c_void,
                                      path: *mut ::libc::c_void,
                                      style: ::libc::c_int) -> ();
    pub fn wxGraphicsContext_StrokePath(_self: *mut ::libc::c_void,
                                        path: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_GetNativeContext(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsContext_GetTextExtent(_self: *mut ::libc::c_void,
                                           text: *mut ::libc::c_void,
                                           width: *mut ::libc::c_double,
                                           height: *mut ::libc::c_double,
                                           descent: *mut ::libc::c_double,
                                           externalLeading:
                                               *mut ::libc::c_double) -> ();
    pub fn wxGraphicsContext_Rotate(_self: *mut ::libc::c_void,
                                    angle: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_Scale(_self: *mut ::libc::c_void,
                                   x: ::libc::c_double, y: ::libc::c_double)
     -> ();
    pub fn wxGraphicsContext_Translate(_self: *mut ::libc::c_void,
                                       dx: ::libc::c_double,
                                       dy: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_SetTransform(_self: *mut ::libc::c_void,
                                          path: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_ConcatTransform(_self: *mut ::libc::c_void,
                                             path: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_SetBrush(_self: *mut ::libc::c_void,
                                      brush: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_SetGraphicsBrush(_self: *mut ::libc::c_void,
                                              brush: *mut ::libc::c_void)
     -> ();
    pub fn wxGraphicsContext_SetFont(_self: *mut ::libc::c_void,
                                     font: *mut ::libc::c_void,
                                     colour: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_SetGraphicsFont(_self: *mut ::libc::c_void,
                                             font: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_SetPen(_self: *mut ::libc::c_void,
                                    pen: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_SetGraphicsPen(_self: *mut ::libc::c_void,
                                            pen: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsContext_StrokeLine(_self: *mut ::libc::c_void,
                                        x1: ::libc::c_double,
                                        y1: ::libc::c_double,
                                        x2: ::libc::c_double,
                                        y2: ::libc::c_double) -> ();
    pub fn wxGraphicsContext_StrokeLines(_self: *mut ::libc::c_void,
                                         n: size_t, x: *mut ::libc::c_void,
                                         y: *mut ::libc::c_void,
                                         style: ::libc::c_int) -> ();
    pub fn wxGraphicsFont_Create() -> *mut ::libc::c_void;
    pub fn wxGraphicsFont_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsMatrix_Create() -> *mut ::libc::c_void;
    pub fn wxGraphicsMatrix_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsMatrix_Concat(_self: *mut ::libc::c_void,
                                   t: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsMatrix_Get(_self: *mut ::libc::c_void,
                                a: *mut ::libc::c_double,
                                b: *mut ::libc::c_double,
                                c: *mut ::libc::c_double,
                                d: *mut ::libc::c_double,
                                tx: *mut ::libc::c_double,
                                ty: *mut ::libc::c_double) -> ();
    pub fn wxGraphicsMatrix_GetNativeMatrix(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsMatrix_Invert(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsMatrix_IsEqual(_self: *mut ::libc::c_void,
                                    t: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGraphicsMatrix_IsIdentity(_self: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGraphicsMatrix_Rotate(_self: *mut ::libc::c_void,
                                   angle: ::libc::c_double) -> ();
    pub fn wxGraphicsMatrix_Scale(_self: *mut ::libc::c_void,
                                  x: ::libc::c_double, y: ::libc::c_double)
     -> ();
    pub fn wxGraphicsMatrix_Set(_self: *mut ::libc::c_void,
                                a: ::libc::c_double, b: ::libc::c_double,
                                c: ::libc::c_double, d: ::libc::c_double,
                                tx: ::libc::c_double, ty: ::libc::c_double)
     -> ();
    pub fn wxGraphicsMatrix_Translate(_self: *mut ::libc::c_void,
                                      dx: ::libc::c_double,
                                      dy: ::libc::c_double) -> ();
    pub fn wxGraphicsMatrix_TransformPoint(_self: *mut ::libc::c_void,
                                           x: *mut ::libc::c_double,
                                           y: *mut ::libc::c_double) -> ();
    pub fn wxGraphicsMatrix_TransformDistance(_self: *mut ::libc::c_void,
                                              dx: *mut ::libc::c_double,
                                              dy: *mut ::libc::c_double)
     -> ();
    pub fn wxGraphicsObject_GetRenderer() -> *mut ::libc::c_void;
    pub fn wxGraphicsObject_IsNull(_self: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGraphicsPath_Create() -> *mut ::libc::c_void;
    pub fn wxGraphicsPath_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsPath_MoveToPoint(_self: *mut ::libc::c_void,
                                      x: ::libc::c_double,
                                      y: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddArc(_self: *mut ::libc::c_void,
                                 x: ::libc::c_double, y: ::libc::c_double,
                                 r: ::libc::c_double,
                                 startAngle: ::libc::c_double,
                                 endAngle: ::libc::c_double,
                                 clockwise: ::libc::c_int) -> ();
    pub fn wxGraphicsPath_AddArcToPoint(_self: *mut ::libc::c_void,
                                        x1: ::libc::c_double,
                                        y1: ::libc::c_double,
                                        x2: ::libc::c_double,
                                        y2: ::libc::c_double,
                                        r: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddCircle(_self: *mut ::libc::c_void,
                                    x: ::libc::c_double, y: ::libc::c_double,
                                    r: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddCurveToPoint(_self: *mut ::libc::c_void,
                                          cx1: ::libc::c_double,
                                          cy1: ::libc::c_double,
                                          cx2: ::libc::c_double,
                                          cy2: ::libc::c_double,
                                          x: ::libc::c_double,
                                          y: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddEllipse(_self: *mut ::libc::c_void,
                                     x: ::libc::c_double, y: ::libc::c_double,
                                     w: ::libc::c_double, h: ::libc::c_double)
     -> ();
    pub fn wxGraphicsPath_AddLineToPoint(_self: *mut ::libc::c_void,
                                         x: ::libc::c_double,
                                         y: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddPath(_self: *mut ::libc::c_void,
                                  x: ::libc::c_double, y: ::libc::c_double,
                                  path: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsPath_AddQuadCurveToPoint(_self: *mut ::libc::c_void,
                                              cx: ::libc::c_double,
                                              cy: ::libc::c_double,
                                              x: ::libc::c_double,
                                              y: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddRectangle(_self: *mut ::libc::c_void,
                                       x: ::libc::c_double,
                                       y: ::libc::c_double,
                                       w: ::libc::c_double,
                                       h: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_AddRoundedRectangle(_self: *mut ::libc::c_void,
                                              x: ::libc::c_double,
                                              y: ::libc::c_double,
                                              w: ::libc::c_double,
                                              h: ::libc::c_double,
                                              radius: ::libc::c_double) -> ();
    pub fn wxGraphicsPath_CloseSubpath(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsPath_Contains(_self: *mut ::libc::c_void,
                                   x: ::libc::c_double, y: ::libc::c_double,
                                   style: ::libc::c_int) -> ();
    pub fn wxGraphicsPath_GetBox(_self: *mut ::libc::c_void,
                                 x: *mut ::libc::c_double,
                                 y: *mut ::libc::c_double,
                                 w: *mut ::libc::c_double,
                                 h: *mut ::libc::c_double) -> ();
    pub fn wxGraphicsPath_GetCurrentPoint(_self: *mut ::libc::c_void,
                                          x: *mut ::libc::c_double,
                                          y: *mut ::libc::c_double) -> ();
    pub fn wxGraphicsPath_Transform(_self: *mut ::libc::c_void,
                                    matrix: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsPath_GetNativePath(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsPath_UnGetNativePath(p: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsPen_Create() -> *mut ::libc::c_void;
    pub fn wxGraphicsPen_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsRenderer_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxGraphicsRenderer_GetDefaultRenderer(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsRenderer_CreateContext(dc: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsRenderer_CreateContextFromWindow(window:
                                                          *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsRenderer_CreateContextFromNativeContext(context:
                                                                 *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGraphicsRenderer_CreateContextFromNativeWindow(window:
                                                                *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGLCanvas_Create(parent: *mut ::libc::c_void,
                             windowID: ::libc::c_int,
                             attributes: *mut ::libc::c_int, x: ::libc::c_int,
                             y: ::libc::c_int, w: ::libc::c_int,
                             h: ::libc::c_int, style: ::libc::c_int,
                             title: *mut ::libc::c_void,
                             palette: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGLCanvas_SetColour(_self: *mut ::libc::c_void,
                                colour: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGLCanvas_SetCurrent(_self: *mut ::libc::c_void,
                                 ctxt: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxGLCanvas_SwapBuffers(_self: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGLCanvas_IsDisplaySupported(attributes: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxGLCanvas_IsExtensionSupported(extension: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxGLContext_Create(win: *mut ::libc::c_void,
                              other: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGLContext_CreateFromNull(win: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxGLContext_SetCurrent(_self: *mut ::libc::c_void,
                                  win: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSound_Create(fileName: *mut ::libc::c_void,
                          isResource: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxSound_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxSound_IsOk(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxSound_Play(_self: *mut ::libc::c_void, flag: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxSound_Stop(_self: *mut ::libc::c_void) -> ();
    pub fn wxManagedPtr_GetPtr(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_NoFinalize(_self: *mut ::libc::c_void) -> ();
    pub fn wxManagedPtr_Finalize(_self: *mut ::libc::c_void) -> ();
    pub fn wxManagedPtr_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxManagedPtr_CreateFromObject(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromDateTime(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromGridCellCoordsArray(obj:
                                                          *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromBitmap(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromIcon(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromBrush(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromColour(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromCursor(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromFont(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxManagedPtr_CreateFromPen(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxObject_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxIcon_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxBrush_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxColour_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxCursor_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxFont_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxPen_SafeDelete(_self: *mut ::libc::c_void) -> ();
    pub fn wxBitmap_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxIcon_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxBrush_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxColour_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxCursor_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFont_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPen_IsStatic(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_Create(parent: *mut ::libc::c_void,
                              windowID: ::libc::c_int,
                              fileName: *mut ::libc::c_void, x: ::libc::c_int,
                              y: ::libc::c_int, w: ::libc::c_int,
                              h: ::libc::c_int, style: ::libc::c_long,
                              szBackend: *mut ::libc::c_void,
                              name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMediaCtrl_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxMediaCtrl_GetBestSize(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMediaCtrl_GetPlaybackRate(_self: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxMediaCtrl_GetVolume(_self: *mut ::libc::c_void)
     -> ::libc::c_double;
    pub fn wxMediaCtrl_GetState(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_Length(_self: *mut ::libc::c_void) -> int64_t;
    pub fn wxMediaCtrl_Load(_self: *mut ::libc::c_void,
                            fileName: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_LoadURI(_self: *mut ::libc::c_void,
                               uri: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_LoadURIWithProxy(_self: *mut ::libc::c_void,
                                        uri: *mut ::libc::c_void,
                                        proxy: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMediaCtrl_Pause(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_Play(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_Seek(_self: *mut ::libc::c_void, offsetWhere: int64_t,
                            mode: ::libc::c_int) -> int64_t;
    pub fn wxMediaCtrl_SetPlaybackRate(_self: *mut ::libc::c_void,
                                       dRate: ::libc::c_double)
     -> ::libc::c_int;
    pub fn wxMediaCtrl_SetVolume(_self: *mut ::libc::c_void,
                                 dVolume: ::libc::c_double) -> ::libc::c_int;
    pub fn wxMediaCtrl_ShowPlayerControls(_self: *mut ::libc::c_void,
                                          flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxMediaCtrl_Stop(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxMediaCtrl_Tell(_self: *mut ::libc::c_void) -> int64_t;
    pub fn wxPreviewFrame_Create(preview: *mut ::libc::c_void,
                                 parent: *mut ::libc::c_void,
                                 title: *mut ::libc::c_void, x: ::libc::c_int,
                                 y: ::libc::c_int, width: ::libc::c_int,
                                 height: ::libc::c_int, style: ::libc::c_int,
                                 name: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPreviewFrame_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxPreviewFrame_Initialize(_self: *mut ::libc::c_void) -> ();
    pub fn expEVT_PRINT_BEGIN() -> ::libc::c_int;
    pub fn expEVT_PRINT_BEGIN_DOC() -> ::libc::c_int;
    pub fn expEVT_PRINT_END() -> ::libc::c_int;
    pub fn expEVT_PRINT_END_DOC() -> ::libc::c_int;
    pub fn expEVT_PRINT_PREPARE() -> ::libc::c_int;
    pub fn expEVT_PRINT_PAGE() -> ::libc::c_int;
    pub fn wxPrintout_GetDC(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxPrintout_GetPPIPrinter(_obj: *mut ::libc::c_void,
                                    _x: *mut ::libc::c_void,
                                    _y: *mut ::libc::c_void) -> ();
    pub fn wxPrintout_GetPPIScreen(_obj: *mut ::libc::c_void,
                                   _x: *mut ::libc::c_void,
                                   _y: *mut ::libc::c_void) -> ();
    pub fn wxPrintout_GetPageSizeMM(_obj: *mut ::libc::c_void,
                                    _w: *mut ::libc::c_void,
                                    _h: *mut ::libc::c_void) -> ();
    pub fn wxPrintout_GetPageSizePixels(_obj: *mut ::libc::c_void,
                                        _w: *mut ::libc::c_void,
                                        _h: *mut ::libc::c_void) -> ();
    pub fn wxPrintout_GetTitle(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxPrintout_IsPreview(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxPrintout_SetDC(_obj: *mut ::libc::c_void,
                            dc: *mut ::libc::c_void) -> ();
    pub fn wxPrintout_SetPPIPrinter(_obj: *mut ::libc::c_void,
                                    x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxPrintout_SetPPIScreen(_obj: *mut ::libc::c_void,
                                   x: ::libc::c_int, y: ::libc::c_int) -> ();
    pub fn wxPrintout_SetPageSizeMM(_obj: *mut ::libc::c_void,
                                    w: ::libc::c_int, h: ::libc::c_int) -> ();
    pub fn wxPrintout_SetPageSizePixels(_obj: *mut ::libc::c_void,
                                        w: ::libc::c_int, h: ::libc::c_int)
     -> ();
    pub fn wxcPrintout_Create(title: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcPrintout_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxcPrintout_SetPageLimits(_self: *mut ::libc::c_void,
                                     startPage: ::libc::c_int,
                                     endPage: ::libc::c_int,
                                     fromPage: ::libc::c_int,
                                     toPage: ::libc::c_int) -> ();
    pub fn wxcPrintout_GetEvtHandler(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcPrintEvent_GetPrintout(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcPrintEvent_GetPage(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxcPrintEvent_GetEndPage(_self: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxcPrintEvent_GetContinue(_self: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxcPrintEvent_SetContinue(_self: *mut ::libc::c_void,
                                     cont: ::libc::c_int) -> ();
    pub fn wxcPrintEvent_SetPageLimits(_self: *mut ::libc::c_void,
                                       startPage: ::libc::c_int,
                                       endPage: ::libc::c_int,
                                       fromPage: ::libc::c_int,
                                       toPage: ::libc::c_int) -> ();
    pub fn wxInputStream_CanRead(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextInputStream_Create(inputStream: *mut ::libc::c_void,
                                    sep: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextInputStream_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxTextInputStream_ReadLine(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextOutputStream_Create(outputStream: *mut ::libc::c_void,
                                     mode: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxTextOutputStream_Delete(_self: *mut ::libc::c_void) -> ();
    pub fn wxTextOutputStream_WriteString(_self: *mut ::libc::c_void,
                                          txt: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AddText(_obj: *mut ::libc::c_void,
                                    text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AddStyledText(_obj: *mut ::libc::c_void,
                                          data: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_InsertText(_obj: *mut ::libc::c_void,
                                       pos: ::libc::c_int,
                                       text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_ClearAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_ClearDocumentStyle(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_GetLength(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetCharAt(_obj: *mut ::libc::c_void,
                                      pos: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetCurrentPos(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetAnchor(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetStyleAt(_obj: *mut ::libc::c_void,
                                       pos: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_Redo(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetUndoCollection(_obj: *mut ::libc::c_void,
                                              collectUndo: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_SelectAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetSavePoint(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_CanRedo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_MarkerLineFromHandle(_obj: *mut ::libc::c_void,
                                                 handle: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_MarkerDeleteHandle(_obj: *mut ::libc::c_void,
                                               handle: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetUndoCollection(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetViewWhiteSpace(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetViewWhiteSpace(_obj: *mut ::libc::c_void,
                                              viewWS: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_PositionFromPoint(_obj: *mut ::libc::c_void,
                                              pt_x: ::libc::c_int,
                                              pt_y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_PositionFromPointClose(_obj: *mut ::libc::c_void,
                                                   x: ::libc::c_int,
                                                   y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GotoLine(_obj: *mut ::libc::c_void,
                                     line: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GotoPos(_obj: *mut ::libc::c_void,
                                    pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetAnchor(_obj: *mut ::libc::c_void,
                                      posAnchor: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetEndStyled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_ConvertEOLs(_obj: *mut ::libc::c_void,
                                        eolMode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetEOLMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetEOLMode(_obj: *mut ::libc::c_void,
                                       eolMode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StartStyling(_obj: *mut ::libc::c_void,
                                         pos: ::libc::c_int,
                                         mask: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetStyling(_obj: *mut ::libc::c_void,
                                       length: ::libc::c_int,
                                       style: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetBufferedDraw(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetBufferedDraw(_obj: *mut ::libc::c_void,
                                            buffered: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetTabWidth(_obj: *mut ::libc::c_void,
                                        tabWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetTabWidth(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetCodePage(_obj: *mut ::libc::c_void,
                                        codePage: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_MarkerDefine(_obj: *mut ::libc::c_void,
                                         markerNumber: ::libc::c_int,
                                         markerSymbol: ::libc::c_int,
                                         foreground_r: uint8_t,
                                         foreground_g: uint8_t,
                                         foreground_b: uint8_t,
                                         background_r: uint8_t,
                                         background_g: uint8_t,
                                         background_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_MarkerSetForeground(_obj: *mut ::libc::c_void,
                                                markerNumber: ::libc::c_int,
                                                fore_r: uint8_t,
                                                fore_g: uint8_t,
                                                fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_MarkerSetBackground(_obj: *mut ::libc::c_void,
                                                markerNumber: ::libc::c_int,
                                                back_r: uint8_t,
                                                back_g: uint8_t,
                                                back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_MarkerAdd(_obj: *mut ::libc::c_void,
                                      line: ::libc::c_int,
                                      markerNumber: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_MarkerDelete(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int,
                                         markerNumber: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_MarkerDeleteAll(_obj: *mut ::libc::c_void,
                                            markerNumber: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_MarkerGet(_obj: *mut ::libc::c_void,
                                      line: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_MarkerNext(_obj: *mut ::libc::c_void,
                                       lineStart: ::libc::c_int,
                                       markerMask: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_MarkerPrevious(_obj: *mut ::libc::c_void,
                                           lineStart: ::libc::c_int,
                                           markerMask: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_MarkerDefineBitmap(_obj: *mut ::libc::c_void,
                                               markerNumber: ::libc::c_int,
                                               bmp: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_SetMarginType(_obj: *mut ::libc::c_void,
                                          margin: ::libc::c_int,
                                          marginType: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetMarginType(_obj: *mut ::libc::c_void,
                                          margin: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMarginWidth(_obj: *mut ::libc::c_void,
                                           margin: ::libc::c_int,
                                           pixelWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetMarginWidth(_obj: *mut ::libc::c_void,
                                           margin: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMarginMask(_obj: *mut ::libc::c_void,
                                          margin: ::libc::c_int,
                                          mask: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetMarginMask(_obj: *mut ::libc::c_void,
                                          margin: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMarginSensitive(_obj: *mut ::libc::c_void,
                                               margin: ::libc::c_int,
                                               sensitive: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetMarginSensitive(_obj: *mut ::libc::c_void,
                                               margin: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_StyleClearAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_StyleSetForeground(_obj: *mut ::libc::c_void,
                                               style: ::libc::c_int,
                                               fore_r: uint8_t,
                                               fore_g: uint8_t,
                                               fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_StyleSetBackground(_obj: *mut ::libc::c_void,
                                               style: ::libc::c_int,
                                               back_r: uint8_t,
                                               back_g: uint8_t,
                                               back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_StyleSetBold(_obj: *mut ::libc::c_void,
                                         style: ::libc::c_int,
                                         bold: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleSetItalic(_obj: *mut ::libc::c_void,
                                           style: ::libc::c_int,
                                           italic: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleSetSize(_obj: *mut ::libc::c_void,
                                         style: ::libc::c_int,
                                         sizePoints: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleSetFaceName(_obj: *mut ::libc::c_void,
                                             style: ::libc::c_int,
                                             fontName: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_StyleSetEOLFilled(_obj: *mut ::libc::c_void,
                                              style: ::libc::c_int,
                                              filled: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleResetDefault(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_StyleSetUnderline(_obj: *mut ::libc::c_void,
                                              style: ::libc::c_int,
                                              underline: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleSetCase(_obj: *mut ::libc::c_void,
                                         style: ::libc::c_int,
                                         caseForce: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleSetCharacterSet(_obj: *mut ::libc::c_void,
                                                 style: ::libc::c_int,
                                                 characterSet: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_StyleSetHotSpot(_obj: *mut ::libc::c_void,
                                            style: ::libc::c_int,
                                            hotspot: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetSelForeground(_obj: *mut ::libc::c_void,
                                             useSetting: ::libc::c_int,
                                             fore_r: uint8_t, fore_g: uint8_t,
                                             fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetSelBackground(_obj: *mut ::libc::c_void,
                                             useSetting: ::libc::c_int,
                                             back_r: uint8_t, back_g: uint8_t,
                                             back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetCaretForeground(_obj: *mut ::libc::c_void,
                                               fore_r: uint8_t,
                                               fore_g: uint8_t,
                                               fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_CmdKeyAssign(_obj: *mut ::libc::c_void,
                                         key: ::libc::c_int,
                                         modifiers: ::libc::c_int,
                                         cmd: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_CmdKeyClear(_obj: *mut ::libc::c_void,
                                        key: ::libc::c_int,
                                        modifiers: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_CmdKeyClearAll(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetStyleBytes(_obj: *mut ::libc::c_void,
                                          length: ::libc::c_int,
                                          styleBytes: *mut ::libc::c_char)
     -> ();
    pub fn wxStyledTextCtrl_StyleSetVisible(_obj: *mut ::libc::c_void,
                                            style: ::libc::c_int,
                                            visible: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetCaretPeriod(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetCaretPeriod(_obj: *mut ::libc::c_void,
                                           periodMilliseconds: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_SetWordChars(_obj: *mut ::libc::c_void,
                                         characters: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_BeginUndoAction(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_EndUndoAction(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_IndicatorSetStyle(_obj: *mut ::libc::c_void,
                                              indic: ::libc::c_int,
                                              style: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_IndicatorGetStyle(_obj: *mut ::libc::c_void,
                                              indic: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_IndicatorSetForeground(_obj: *mut ::libc::c_void,
                                                   indic: ::libc::c_int,
                                                   fore_r: uint8_t,
                                                   fore_g: uint8_t,
                                                   fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetWhitespaceForeground(_obj: *mut ::libc::c_void,
                                                    useSetting: ::libc::c_int,
                                                    fore_r: uint8_t,
                                                    fore_g: uint8_t,
                                                    fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetWhitespaceBackground(_obj: *mut ::libc::c_void,
                                                    useSetting: ::libc::c_int,
                                                    back_r: uint8_t,
                                                    back_g: uint8_t,
                                                    back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetStyleBits(_obj: *mut ::libc::c_void,
                                         bits: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetStyleBits(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetLineState(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int,
                                         state: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetLineState(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetMaxLineState(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetCaretLineVisible(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetCaretLineVisible(_obj: *mut ::libc::c_void,
                                                show: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_StyleSetChangeable(_obj: *mut ::libc::c_void,
                                               style: ::libc::c_int,
                                               changeable: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompShow(_obj: *mut ::libc::c_void,
                                         lenEntered: ::libc::c_int,
                                         itemList: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AutoCompCancel(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AutoCompActive(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompPosStart(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompComplete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AutoCompStops(_obj: *mut ::libc::c_void,
                                          characterSet: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompSetSeparator(_obj: *mut ::libc::c_void,
                                                 separatorCharacter:
                                                     ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_AutoCompGetSeparator(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompSelect(_obj: *mut ::libc::c_void,
                                           text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AutoCompSetCancelAtStart(_obj:
                                                         *mut ::libc::c_void,
                                                     cancel: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompGetCancelAtStart(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompSetFillUps(_obj: *mut ::libc::c_void,
                                               characterSet:
                                                   *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AutoCompSetChooseSingle(_obj: *mut ::libc::c_void,
                                                    chooseSingle:
                                                        ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_AutoCompGetChooseSingle(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompSetIgnoreCase(_obj: *mut ::libc::c_void,
                                                  ignoreCase: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompGetIgnoreCase(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_UserListShow(_obj: *mut ::libc::c_void,
                                         listType: ::libc::c_int,
                                         itemList: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_AutoCompSetAutoHide(_obj: *mut ::libc::c_void,
                                                autoHide: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompGetAutoHide(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompSetDropRestOfWord(_obj:
                                                          *mut ::libc::c_void,
                                                      dropRestOfWord:
                                                          ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompGetDropRestOfWord(_obj:
                                                          *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_RegisterImage(_obj: *mut ::libc::c_void,
                                          _type: ::libc::c_int,
                                          bmp: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_ClearRegisteredImages(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_AutoCompGetTypeSeparator(_obj:
                                                         *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AutoCompSetTypeSeparator(_obj:
                                                         *mut ::libc::c_void,
                                                     separatorCharacter:
                                                         ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetIndent(_obj: *mut ::libc::c_void,
                                      indentSize: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetIndent(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetUseTabs(_obj: *mut ::libc::c_void,
                                       useTabs: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetUseTabs(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetLineIndentation(_obj: *mut ::libc::c_void,
                                               line: ::libc::c_int,
                                               indentSize: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetLineIndentation(_obj: *mut ::libc::c_void,
                                               line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetLineIndentPosition(_obj: *mut ::libc::c_void,
                                                  line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetColumn(_obj: *mut ::libc::c_void,
                                      pos: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetUseHorizontalScrollBar(_obj:
                                                          *mut ::libc::c_void,
                                                      show: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetUseHorizontalScrollBar(_obj:
                                                          *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetIndentationGuides(_obj: *mut ::libc::c_void,
                                                 show: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetIndentationGuides(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetHighlightGuide(_obj: *mut ::libc::c_void,
                                              column: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetHighlightGuide(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetLineEndPosition(_obj: *mut ::libc::c_void,
                                               line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetCodePage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetReadOnly(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetCurrentPos(_obj: *mut ::libc::c_void,
                                          pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetSelectionStart(_obj: *mut ::libc::c_void,
                                              pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetSelectionStart(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetSelectionEnd(_obj: *mut ::libc::c_void,
                                            pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetSelectionEnd(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetPrintMagnification(_obj: *mut ::libc::c_void,
                                                  magnification:
                                                      ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetPrintMagnification(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetPrintColourMode(_obj: *mut ::libc::c_void,
                                               mode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetPrintColourMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_FindText(_obj: *mut ::libc::c_void,
                                     minPos: ::libc::c_int,
                                     maxPos: ::libc::c_int,
                                     text: *mut ::libc::c_void,
                                     flags: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_FormatRange(_obj: *mut ::libc::c_void,
                                        doDraw: ::libc::c_int,
                                        startPos: ::libc::c_int,
                                        endPos: ::libc::c_int,
                                        draw: *mut ::libc::c_void,
                                        target: *mut ::libc::c_void,
                                        renderRect: *mut ::libc::c_void,
                                        pageRect: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetFirstVisibleLine(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetLineCount(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMarginLeft(_obj: *mut ::libc::c_void,
                                          pixelWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetMarginLeft(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMarginRight(_obj: *mut ::libc::c_void,
                                           pixelWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetMarginRight(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetModify(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetSelection(_obj: *mut ::libc::c_void,
                                         start: ::libc::c_int,
                                         end: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_HideSelection(_obj: *mut ::libc::c_void,
                                          normal: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_LineFromPosition(_obj: *mut ::libc::c_void,
                                             pos: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_PositionFromLine(_obj: *mut ::libc::c_void,
                                             line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_LineScroll(_obj: *mut ::libc::c_void,
                                       columns: ::libc::c_int,
                                       lines: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_EnsureCaretVisible(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_ReplaceSelection(_obj: *mut ::libc::c_void,
                                             text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetReadOnly(_obj: *mut ::libc::c_void,
                                        readOnly: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_CanPaste(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_CanUndo(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_EmptyUndoBuffer(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_Undo(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_Cut(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_Copy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_Paste(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_Clear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetText(_obj: *mut ::libc::c_void,
                                    text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_GetTextLength(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetOvertype(_obj: *mut ::libc::c_void,
                                        overtype: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetOvertype(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetCaretWidth(_obj: *mut ::libc::c_void,
                                          pixelWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetCaretWidth(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetTargetStart(_obj: *mut ::libc::c_void,
                                           pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetTargetStart(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetTargetEnd(_obj: *mut ::libc::c_void,
                                         pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetTargetEnd(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_ReplaceTarget(_obj: *mut ::libc::c_void,
                                          text: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_ReplaceTargetRE(_obj: *mut ::libc::c_void,
                                            text: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SearchInTarget(_obj: *mut ::libc::c_void,
                                           text: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetSearchFlags(_obj: *mut ::libc::c_void,
                                           flags: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetSearchFlags(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_CallTipShow(_obj: *mut ::libc::c_void,
                                        pos: ::libc::c_int,
                                        definition: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_CallTipCancel(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_CallTipActive(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_CallTipPosAtStart(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_CallTipSetHighlight(_obj: *mut ::libc::c_void,
                                                start: ::libc::c_int,
                                                end: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_CallTipSetBackground(_obj: *mut ::libc::c_void,
                                                 back_r: uint8_t,
                                                 back_g: uint8_t,
                                                 back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_CallTipSetForeground(_obj: *mut ::libc::c_void,
                                                 fore_r: uint8_t,
                                                 fore_g: uint8_t,
                                                 fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_CallTipSetForegroundHighlight(_obj:
                                                              *mut ::libc::c_void,
                                                          fore_r: uint8_t,
                                                          fore_g: uint8_t,
                                                          fore_b: uint8_t)
     -> ();
    pub fn wxStyledTextCtrl_VisibleFromDocLine(_obj: *mut ::libc::c_void,
                                               line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_DocLineFromVisible(_obj: *mut ::libc::c_void,
                                               lineDisplay: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetFoldLevel(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int,
                                         level: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetFoldLevel(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetLastChild(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int,
                                         level: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetFoldParent(_obj: *mut ::libc::c_void,
                                          line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_ShowLines(_obj: *mut ::libc::c_void,
                                      lineStart: ::libc::c_int,
                                      lineEnd: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_HideLines(_obj: *mut ::libc::c_void,
                                      lineStart: ::libc::c_int,
                                      lineEnd: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetLineVisible(_obj: *mut ::libc::c_void,
                                           line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetFoldExpanded(_obj: *mut ::libc::c_void,
                                            line: ::libc::c_int,
                                            expanded: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetFoldExpanded(_obj: *mut ::libc::c_void,
                                            line: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_ToggleFold(_obj: *mut ::libc::c_void,
                                       line: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_EnsureVisible(_obj: *mut ::libc::c_void,
                                          line: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetFoldFlags(_obj: *mut ::libc::c_void,
                                         flags: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_EnsureVisibleEnforcePolicy(_obj:
                                                           *mut ::libc::c_void,
                                                       line: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_SetTabIndents(_obj: *mut ::libc::c_void,
                                          tabIndents: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetTabIndents(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetBackSpaceUnIndents(_obj: *mut ::libc::c_void,
                                                  bsUnIndents: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetBackSpaceUnIndents(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMouseDwellTime(_obj: *mut ::libc::c_void,
                                              periodMilliseconds:
                                                  ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetMouseDwellTime(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_WordStartPosition(_obj: *mut ::libc::c_void,
                                              pos: ::libc::c_int,
                                              onlyWordCharacters:
                                                  ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_WordEndPosition(_obj: *mut ::libc::c_void,
                                            pos: ::libc::c_int,
                                            onlyWordCharacters: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetWrapMode(_obj: *mut ::libc::c_void,
                                        mode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetWrapMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetLayoutCache(_obj: *mut ::libc::c_void,
                                           mode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetLayoutCache(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetScrollWidth(_obj: *mut ::libc::c_void,
                                           pixelWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetScrollWidth(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_TextWidth(_obj: *mut ::libc::c_void,
                                      style: ::libc::c_int,
                                      text: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetEndAtLastLine(_obj: *mut ::libc::c_void,
                                             endAtLastLine: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetEndAtLastLine(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_TextHeight(_obj: *mut ::libc::c_void,
                                       line: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetUseVerticalScrollBar(_obj: *mut ::libc::c_void,
                                                    show: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetUseVerticalScrollBar(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AppendText(_obj: *mut ::libc::c_void,
                                       text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_GetTwoPhaseDraw(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetTwoPhaseDraw(_obj: *mut ::libc::c_void,
                                            twoPhase: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_TargetFromSelection(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_LinesJoin(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_LinesSplit(_obj: *mut ::libc::c_void,
                                       pixelWidth: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetFoldMarginColour(_obj: *mut ::libc::c_void,
                                                useSetting: ::libc::c_int,
                                                back_r: uint8_t,
                                                back_g: uint8_t,
                                                back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetFoldMarginHiColour(_obj: *mut ::libc::c_void,
                                                  useSetting: ::libc::c_int,
                                                  fore_r: uint8_t,
                                                  fore_g: uint8_t,
                                                  fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_LineDuplicate(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_HomeDisplay(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_HomeDisplayExtend(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_LineEndDisplay(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_LineEndDisplayExtend(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_LineCopy(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_MoveCaretInsideView(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_LineLength(_obj: *mut ::libc::c_void,
                                       line: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_BraceHighlight(_obj: *mut ::libc::c_void,
                                           pos1: ::libc::c_int,
                                           pos2: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_BraceBadLight(_obj: *mut ::libc::c_void,
                                          pos: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_BraceMatch(_obj: *mut ::libc::c_void,
                                       pos: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStyledTextCtrl_GetViewEOL(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetViewEOL(_obj: *mut ::libc::c_void,
                                       visible: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetDocPointer(_obj: *mut ::libc::c_void,
                                          docPointer: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_SetModEventMask(_obj: *mut ::libc::c_void,
                                            mask: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetEdgeColumn(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetEdgeColumn(_obj: *mut ::libc::c_void,
                                          column: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetEdgeMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetEdgeMode(_obj: *mut ::libc::c_void,
                                        mode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetEdgeColour(_obj: *mut ::libc::c_void,
                                          edgeColour_r: uint8_t,
                                          edgeColour_g: uint8_t,
                                          edgeColour_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SearchAnchor(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SearchNext(_obj: *mut ::libc::c_void,
                                       flags: ::libc::c_int,
                                       text: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SearchPrev(_obj: *mut ::libc::c_void,
                                       flags: ::libc::c_int,
                                       text: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_LinesOnScreen(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_UsePopUp(_obj: *mut ::libc::c_void,
                                     allowPopUp: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SelectionIsRectangle(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetZoom(_obj: *mut ::libc::c_void,
                                    zoom: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetZoom(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_AddRefDocument(_obj: *mut ::libc::c_void,
                                           docPointer: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_ReleaseDocument(_obj: *mut ::libc::c_void,
                                            docPointer: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_GetModEventMask(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetSTCFocus(_obj: *mut ::libc::c_void,
                                        focus: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetSTCFocus(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetStatus(_obj: *mut ::libc::c_void,
                                      statusCode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetStatus(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetMouseDownCaptures(_obj: *mut ::libc::c_void,
                                                 captures: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_GetMouseDownCaptures(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetSTCCursor(_obj: *mut ::libc::c_void,
                                         cursorType: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetSTCCursor(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetControlCharSymbol(_obj: *mut ::libc::c_void,
                                                 symbol: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetControlCharSymbol(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_WordPartLeft(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_WordPartLeftExtend(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_WordPartRight(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_WordPartRightExtend(_obj: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_SetVisiblePolicy(_obj: *mut ::libc::c_void,
                                             visiblePolicy: ::libc::c_int,
                                             visibleSlop: ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_DelLineLeft(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_DelLineRight(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetXOffset(_obj: *mut ::libc::c_void,
                                       newOffset: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetXOffset(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_ChooseCaretX(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetXCaretPolicy(_obj: *mut ::libc::c_void,
                                            caretPolicy: ::libc::c_int,
                                            caretSlop: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetYCaretPolicy(_obj: *mut ::libc::c_void,
                                            caretPolicy: ::libc::c_int,
                                            caretSlop: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetPrintWrapMode(_obj: *mut ::libc::c_void,
                                             mode: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetPrintWrapMode(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetHotspotActiveForeground(_obj:
                                                           *mut ::libc::c_void,
                                                       useSetting:
                                                           ::libc::c_int,
                                                       fore_r: uint8_t,
                                                       fore_g: uint8_t,
                                                       fore_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetHotspotActiveBackground(_obj:
                                                           *mut ::libc::c_void,
                                                       useSetting:
                                                           ::libc::c_int,
                                                       back_r: uint8_t,
                                                       back_g: uint8_t,
                                                       back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_SetHotspotActiveUnderline(_obj:
                                                          *mut ::libc::c_void,
                                                      underline:
                                                          ::libc::c_int)
     -> ();
    pub fn wxStyledTextCtrl_PositionBefore(_obj: *mut ::libc::c_void,
                                           pos: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_PositionAfter(_obj: *mut ::libc::c_void,
                                          pos: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_CopyRange(_obj: *mut ::libc::c_void,
                                      start: ::libc::c_int,
                                      end: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_CopyText(_obj: *mut ::libc::c_void,
                                     length: ::libc::c_int,
                                     text: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_StartRecord(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_StopRecord(_obj: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetLexer(_obj: *mut ::libc::c_void,
                                     lexer: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetLexer(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_Colourise(_obj: *mut ::libc::c_void,
                                      start: ::libc::c_int,
                                      end: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetProperty(_obj: *mut ::libc::c_void,
                                        key: *mut ::libc::c_void,
                                        value: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetKeyWords(_obj: *mut ::libc::c_void,
                                        keywordSet: ::libc::c_int,
                                        keyWords: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetLexerLanguage(_obj: *mut ::libc::c_void,
                                             language: *mut ::libc::c_void)
     -> ();
    pub fn wxStyledTextCtrl_GetCurrentLine(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_StyleSetSpec(_obj: *mut ::libc::c_void,
                                         styleNum: ::libc::c_int,
                                         spec: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_StyleSetFont(_obj: *mut ::libc::c_void,
                                         styleNum: ::libc::c_int,
                                         font: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_StyleSetFontAttr(_obj: *mut ::libc::c_void,
                                             styleNum: ::libc::c_int,
                                             size: ::libc::c_int,
                                             faceName: *mut ::libc::c_void,
                                             bold: ::libc::c_int,
                                             italic: ::libc::c_int,
                                             underline: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_CmdKeyExecute(_obj: *mut ::libc::c_void,
                                          cmd: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetMargins(_obj: *mut ::libc::c_void,
                                       left: ::libc::c_int,
                                       right: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_GetSelection(_obj: *mut ::libc::c_void,
                                         startPos: *mut ::libc::c_int,
                                         endPos: *mut ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_ScrollToLine(_obj: *mut ::libc::c_void,
                                         line: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_ScrollToColumn(_obj: *mut ::libc::c_void,
                                           column: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SetVScrollBar(_obj: *mut ::libc::c_void,
                                          bar: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_SetHScrollBar(_obj: *mut ::libc::c_void,
                                          bar: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextCtrl_GetLastKeydownProcessed(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_SetLastKeydownProcessed(_obj: *mut ::libc::c_void,
                                                    val: ::libc::c_int) -> ();
    pub fn wxStyledTextCtrl_SaveFile(_obj: *mut ::libc::c_void,
                                     filename: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_LoadFile(_obj: *mut ::libc::c_void,
                                     filename: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextCtrl_Create(_prt: *mut ::libc::c_void,
                                   _id: ::libc::c_int,
                                   _txt: *mut ::libc::c_void,
                                   _lft: ::libc::c_int, _top: ::libc::c_int,
                                   _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                   style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_IndicatorGetForeground(_obj: *mut ::libc::c_void,
                                                   indic: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetCaretLineBackground(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_SetCaretLineBackground(_obj: *mut ::libc::c_void,
                                                   back_r: uint8_t,
                                                   back_g: uint8_t,
                                                   back_b: uint8_t) -> ();
    pub fn wxStyledTextCtrl_GetCaretForeground(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetLine(_obj: *mut ::libc::c_void,
                                    line: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetTextRange(_obj: *mut ::libc::c_void,
                                         startPos: ::libc::c_int,
                                         endPos: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetSelectedText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_CreateDocument(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetEdgeColour(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_GetDocPointer(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextCtrl_PointFromPosition(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextEvent_GetPosition(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetKey(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetModifiers(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetModificationType(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetLength(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetLinesAdded(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetLine(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetFoldLevelNow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetFoldLevelPrev(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetMargin(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetMessage(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetWParam(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetLParam(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetListType(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetX(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetY(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetDragText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextEvent_GetDragAllowMove(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetDragResult(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetShift(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetControl(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetAlt(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxStyledTextEvent_GetText(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextEvent_Clone(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxStyledTextEvent_SetPosition(_obj: *mut ::libc::c_void,
                                         pos: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetKey(_obj: *mut ::libc::c_void,
                                    k: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetModifiers(_obj: *mut ::libc::c_void,
                                          m: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetModificationType(_obj: *mut ::libc::c_void,
                                                 t: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetText(_obj: *mut ::libc::c_void,
                                     t: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextEvent_SetLength(_obj: *mut ::libc::c_void,
                                       len: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetLinesAdded(_obj: *mut ::libc::c_void,
                                           num: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetLine(_obj: *mut ::libc::c_void,
                                     val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetFoldLevelNow(_obj: *mut ::libc::c_void,
                                             val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetFoldLevelPrev(_obj: *mut ::libc::c_void,
                                              val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetMargin(_obj: *mut ::libc::c_void,
                                       val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetMessage(_obj: *mut ::libc::c_void,
                                        val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetWParam(_obj: *mut ::libc::c_void,
                                       val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetLParam(_obj: *mut ::libc::c_void,
                                       val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetListType(_obj: *mut ::libc::c_void,
                                         val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetX(_obj: *mut ::libc::c_void,
                                  val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetY(_obj: *mut ::libc::c_void,
                                  val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetDragText(_obj: *mut ::libc::c_void,
                                         val: *mut ::libc::c_void) -> ();
    pub fn wxStyledTextEvent_SetDragAllowMove(_obj: *mut ::libc::c_void,
                                              val: ::libc::c_int) -> ();
    pub fn wxStyledTextEvent_SetDragResult(_obj: *mut ::libc::c_void,
                                           val: ::libc::c_int) -> ();
    pub fn expEVT_STC_CHANGE() -> ::libc::c_int;
    pub fn expEVT_STC_STYLENEEDED() -> ::libc::c_int;
    pub fn expEVT_STC_CHARADDED() -> ::libc::c_int;
    pub fn expEVT_STC_SAVEPOINTREACHED() -> ::libc::c_int;
    pub fn expEVT_STC_SAVEPOINTLEFT() -> ::libc::c_int;
    pub fn expEVT_STC_ROMODIFYATTEMPT() -> ::libc::c_int;
    pub fn expEVT_STC_KEY() -> ::libc::c_int;
    pub fn expEVT_STC_DOUBLECLICK() -> ::libc::c_int;
    pub fn expEVT_STC_UPDATEUI() -> ::libc::c_int;
    pub fn expEVT_STC_MODIFIED() -> ::libc::c_int;
    pub fn expEVT_STC_MACRORECORD() -> ::libc::c_int;
    pub fn expEVT_STC_MARGINCLICK() -> ::libc::c_int;
    pub fn expEVT_STC_NEEDSHOWN() -> ::libc::c_int;
    pub fn expEVT_STC_PAINTED() -> ::libc::c_int;
    pub fn expEVT_STC_USERLISTSELECTION() -> ::libc::c_int;
    pub fn expEVT_STC_URIDROPPED() -> ::libc::c_int;
    pub fn expEVT_STC_DWELLSTART() -> ::libc::c_int;
    pub fn expEVT_STC_DWELLEND() -> ::libc::c_int;
    pub fn expEVT_STC_START_DRAG() -> ::libc::c_int;
    pub fn expEVT_STC_DRAG_OVER() -> ::libc::c_int;
    pub fn expEVT_STC_DO_DROP() -> ::libc::c_int;
    pub fn expEVT_STC_ZOOM() -> ::libc::c_int;
    pub fn expEVT_STC_HOTSPOT_CLICK() -> ::libc::c_int;
    pub fn expEVT_STC_HOTSPOT_DCLICK() -> ::libc::c_int;
    pub fn expEVT_STC_CALLTIP_CLICK() -> ::libc::c_int;
    pub fn expEVT_STC_AUTOCOMP_SELECTION() -> ::libc::c_int;
    pub fn wxXmlResource_GetStyledTextCtrl(_obj: *mut ::libc::c_void,
                                           str_id: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClosure_Create(_fun_CEvent: *mut ::libc::c_void,
                            _data: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClosure_GetData(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEvtHandler_GetClosure(_obj: *mut ::libc::c_void,
                                   id: ::libc::c_int, _type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxEvtHandler_GetClientClosure(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxEvtHandler_SetClientClosure(_obj: *mut ::libc::c_void,
                                         closure: *mut ::libc::c_void) -> ();
    pub fn wxObject_GetClientClosure(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxObject_SetClientClosure(_obj: *mut ::libc::c_void,
                                     closure: *mut ::libc::c_void) -> ();
    pub fn wxObject_Delete(obj: *mut ::libc::c_void) -> ();
    pub fn wxFrame_GetTitle(_obj: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn wxFrame_SetTitle(_frame: *mut ::libc::c_void,
                            _txt: *mut ::libc::c_void) -> ();
    pub fn wxFrame_SetShape(_self: *mut ::libc::c_void,
                            region: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFrame_ShowFullScreen(_self: *mut ::libc::c_void,
                                  show: ::libc::c_int, style: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxFrame_IsFullScreen(_self: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxFrame_Centre(_self: *mut ::libc::c_void,
                          orientation: ::libc::c_int) -> ();
    pub fn wxCursor_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxDateTime_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxMouseEvent_GetWheelDelta(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_GetWheelRotation(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxMouseEvent_GetButton(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxcGetMousePosition() -> *mut ::libc::c_void;
    pub fn wxDC_GetUserScaleX(dc: *mut ::libc::c_void) -> ::libc::c_double;
    pub fn wxDC_GetUserScaleY(dc: *mut ::libc::c_void) -> ::libc::c_double;
    pub fn wxWindow_ConvertDialogToPixelsEx(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_ConvertPixelsToDialogEx(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxWindow_ScreenToClient2(_obj: *mut ::libc::c_void,
                                    x: ::libc::c_int, y: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxString_Create(buffer: *mut ::libc::c_char)
     -> *mut ::libc::c_void;
    pub fn wxString_CreateLen(buffer: *mut ::libc::c_char, len: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxString_Delete(s: *mut ::libc::c_void) -> ();
    pub fn wxString_GetString(s: *mut ::libc::c_void,
                              buffer: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn wxString_Length(s: *mut ::libc::c_void) -> size_t;
    pub fn wxMenu_GetMenuBar(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenuBar_GetFrame(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxListEvent_GetCacheFrom(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxListEvent_GetCacheTo(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxListCtrl_AssignImageList(_obj: *mut ::libc::c_void,
                                      images: *mut ::libc::c_void,
                                      which: ::libc::c_int) -> ();
    pub fn wxListCtrl_GetColumn2(_obj: *mut ::libc::c_void,
                                 col: ::libc::c_int,
                                 item: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_GetItem2(_obj: *mut ::libc::c_void,
                               info: *mut ::libc::c_void) -> ();
    pub fn wxListCtrl_GetItemPosition2(_obj: *mut ::libc::c_void,
                                       item: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxListCtrl_SortItems2(_obj: *mut ::libc::c_void,
                                 closure: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxcTreeItemData_Create(closure: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcTreeItemData_GetClientClosure(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcTreeItemData_SetClientClosure(_self: *mut ::libc::c_void,
                                            closure: *mut ::libc::c_void)
     -> ();
    pub fn wxTreeItemId_Clone(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeItemId_CreateFromValue(value: intptr_t)
     -> *mut ::libc::c_void;
    pub fn wxTreeItemId_GetValue(_obj: *mut ::libc::c_void) -> intptr_t;
    pub fn wxTreeEvent_GetKeyEvent(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeEvent_IsEditCancelled(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTreeEvent_Allow(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_Create2(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                              _lft: ::libc::c_int, _top: ::libc::c_int,
                              _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                              _stl: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_InsertItem2(_obj: *mut ::libc::c_void,
                                  parent: *mut ::libc::c_void,
                                  idPrevious: *mut ::libc::c_void,
                                  text: *mut ::libc::c_void,
                                  image: ::libc::c_int,
                                  selectedImage: ::libc::c_int,
                                  closure: *mut ::libc::c_void,
                                  _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_InsertItemByIndex2(_obj: *mut ::libc::c_void,
                                         parent: *mut ::libc::c_void,
                                         index: ::libc::c_int,
                                         text: *mut ::libc::c_void,
                                         image: ::libc::c_int,
                                         selectedImage: ::libc::c_int,
                                         closure: *mut ::libc::c_void,
                                         _item: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_GetItemClientClosure(_obj: *mut ::libc::c_void,
                                           item: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTreeCtrl_SetItemClientClosure(_obj: *mut ::libc::c_void,
                                           item: *mut ::libc::c_void,
                                           closure: *mut ::libc::c_void)
     -> ();
    pub fn wxTreeCtrl_AssignImageList(_obj: *mut ::libc::c_void,
                                      imageList: *mut ::libc::c_void) -> ();
    pub fn wxTreeCtrl_AssignStateImageList(_obj: *mut ::libc::c_void,
                                           imageList: *mut ::libc::c_void)
     -> ();
    pub fn wxDC_GetPixel2(_obj: *mut ::libc::c_void, x: ::libc::c_int,
                          y: ::libc::c_int, col: *mut ::libc::c_void) -> ();
    pub fn wxScrolledWindow_SetScrollRate(_obj: *mut ::libc::c_void,
                                          xstep: ::libc::c_int,
                                          ystep: ::libc::c_int) -> ();
    pub fn wxObject_GetClassInfo(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxObject_IsKindOf(_obj: *mut ::libc::c_void,
                             classInfo: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxObject_IsScrolledWindow(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxClassInfo_FindClass(_txt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClassInfo_GetBaseClassName1(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClassInfo_GetBaseClassName2(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClassInfo_GetClassNameEx(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxClassInfo_GetSize(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxClassInfo_IsKindOfEx(_obj: *mut ::libc::c_void,
                                  classInfo: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxNotebook_AssignImageList(_obj: *mut ::libc::c_void,
                                      imageList: *mut ::libc::c_void) -> ();
    pub fn wxTimerEx_Connect(_obj: *mut ::libc::c_void,
                             closure: *mut ::libc::c_void) -> ();
    pub fn wxTimerEx_Create() -> *mut ::libc::c_void;
    pub fn wxTimerEx_GetClosure(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxMenu_AppendRadioItem(_self: *mut ::libc::c_void,
                                  id: ::libc::c_int,
                                  text: *mut ::libc::c_void,
                                  help: *mut ::libc::c_void) -> ();
    pub fn wxMenuItem_CreateSeparator() -> *mut ::libc::c_void;
    pub fn wxMenuItem_CreateEx(id: ::libc::c_int, label: *mut ::libc::c_void,
                               help: *mut ::libc::c_void,
                               itemkind: ::libc::c_int,
                               submenu: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxToolBar_AddTool2(_obj: *mut ::libc::c_void,
                              toolId: ::libc::c_int,
                              label: *mut ::libc::c_void,
                              bmp: *mut ::libc::c_void,
                              bmpDisabled: *mut ::libc::c_void,
                              itemKind: ::libc::c_int,
                              shortHelp: *mut ::libc::c_void,
                              longHelp: *mut ::libc::c_void) -> ();
    pub fn wxProgressDialog_Create(title: *mut ::libc::c_void,
                                   message: *mut ::libc::c_void,
                                   max: ::libc::c_int,
                                   parent: *mut ::libc::c_void,
                                   style: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxProgressDialog_Update(obj: *mut ::libc::c_void,
                                   value: ::libc::c_int) -> ::libc::c_int;
    pub fn wxProgressDialog_UpdateWithMessage(obj: *mut ::libc::c_void,
                                              value: ::libc::c_int,
                                              message: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxProgressDialog_Resume(obj: *mut ::libc::c_void) -> ();
    pub fn wxVersionNumber() -> ::libc::c_int;
    pub fn wxIsDefined(s: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn wxInputSink_Create(input: *mut ::libc::c_void,
                              evtHandler: *mut ::libc::c_void,
                              bufferLen: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxInputSink_GetId(obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxInputSink_Start(obj: *mut ::libc::c_void) -> ();
    pub fn wxInputSinkEvent_LastError(obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxInputSinkEvent_LastRead(obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxInputSinkEvent_LastInput(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_char;
    pub fn wxcHtmlEvent_GetMouseEvent(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcHtmlEvent_GetHtmlCell(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcHtmlEvent_GetHtmlCellId(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcHtmlEvent_GetHref(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcHtmlEvent_GetTarget(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcHtmlEvent_GetLogicalPosition(_self: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxcHtmlWindow_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                                _lft: ::libc::c_int, _top: ::libc::c_int,
                                _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                                _stl: ::libc::c_int,
                                _txt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_Create(_prt: *mut ::libc::c_void, _id: ::libc::c_int,
                               _lft: ::libc::c_int, _top: ::libc::c_int,
                               _wdt: ::libc::c_int, _hgt: ::libc::c_int,
                               _stl: ::libc::c_int, _txt: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_AppendToPage(_obj: *mut ::libc::c_void,
                                     source: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlWindow_GetInternalRepresentation(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_GetOpenedAnchor(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_GetOpenedPage(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_GetOpenedPageTitle(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_GetRelatedFrame(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxHtmlWindow_HistoryBack(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlWindow_HistoryCanBack(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlWindow_HistoryCanForward(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlWindow_HistoryClear(_obj: *mut ::libc::c_void) -> ();
    pub fn wxHtmlWindow_HistoryForward(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlWindow_LoadPage(_obj: *mut ::libc::c_void,
                                 location: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxHtmlWindow_ReadCustomization(_obj: *mut ::libc::c_void,
                                          cfg: *mut ::libc::c_void,
                                          path: *mut ::libc::c_void) -> ();
    pub fn wxHtmlWindow_SetBorders(_obj: *mut ::libc::c_void,
                                   b: ::libc::c_int) -> ();
    pub fn wxHtmlWindow_SetFonts(_obj: *mut ::libc::c_void,
                                 normal_face: *mut ::libc::c_void,
                                 fixed_face: *mut ::libc::c_void,
                                 sizes: *mut ::libc::c_int) -> ();
    pub fn wxHtmlWindow_SetPage(_obj: *mut ::libc::c_void,
                                source: *mut ::libc::c_void) -> ();
    pub fn wxHtmlWindow_SetRelatedFrame(_obj: *mut ::libc::c_void,
                                        frame: *mut ::libc::c_void,
                                        format: *mut ::libc::c_void) -> ();
    pub fn wxHtmlWindow_SetRelatedStatusBar(_obj: *mut ::libc::c_void,
                                            bar: ::libc::c_int) -> ();
    pub fn wxHtmlWindow_WriteCustomization(_obj: *mut ::libc::c_void,
                                           cfg: *mut ::libc::c_void,
                                           path: *mut ::libc::c_void) -> ();
    pub fn wxGridCellTextEnterEditor_Ctor() -> *mut ::libc::c_void;
    pub fn wxLogStderr_Create() -> *mut ::libc::c_void;
    pub fn wxLogStderr_CreateStdOut() -> *mut ::libc::c_void;
    pub fn wxLogNull_Create() -> *mut ::libc::c_void;
    pub fn wxLogTextCtrl_Create(text: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLogWindow_Create(parent: *mut ::libc::c_void,
                              title: *mut ::libc::c_char,
                              showit: ::libc::c_int,
                              passthrough: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxLogWindow_GetFrame(obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn LogError(_msg: *mut ::libc::c_void) -> ();
    pub fn LogFatalError(_msg: *mut ::libc::c_void) -> ();
    pub fn LogWarning(_msg: *mut ::libc::c_void) -> ();
    pub fn LogMessage(_msg: *mut ::libc::c_void) -> ();
    pub fn LogVerbose(_msg: *mut ::libc::c_void) -> ();
    pub fn LogStatus(_msg: *mut ::libc::c_void) -> ();
    pub fn LogSysError(_msg: *mut ::libc::c_void) -> ();
    pub fn LogDebug(_msg: *mut ::libc::c_void) -> ();
    pub fn LogTrace(mask: *mut ::libc::c_void, _msg: *mut ::libc::c_void)
     -> ();
    pub fn wxLog_AddTraceMask(_obj: *mut ::libc::c_void,
                              str: *mut ::libc::c_void) -> ();
    pub fn wxLog_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLog_DontCreateOnDemand(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLog_Flush(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLog_FlushActive(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLog_GetActiveTarget() -> *mut ::libc::c_void;
    pub fn wxLog_GetTimestamp(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_char;
    pub fn wxLog_GetTraceMask(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxLog_GetVerbose(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxLog_HasPendingMessages(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLog_IsAllowedTraceMask(_obj: *mut ::libc::c_void,
                                    mask: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxLog_OnLog(_obj: *mut ::libc::c_void, level: ::libc::c_int,
                       szString: *mut ::libc::c_void, t: ::libc::c_int) -> ();
    pub fn wxLog_RemoveTraceMask(_obj: *mut ::libc::c_void,
                                 str: *mut ::libc::c_void) -> ();
    pub fn wxLog_Resume(_obj: *mut ::libc::c_void) -> ();
    pub fn wxLog_SetActiveTarget(pLogger: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxLog_SetTimestamp(_obj: *mut ::libc::c_void,
                              ts: *mut ::libc::c_void) -> ();
    pub fn wxLog_SetTraceMask(_obj: *mut ::libc::c_void,
                              ulMask: ::libc::c_int) -> ();
    pub fn wxLog_SetVerbose(_obj: *mut ::libc::c_void,
                            bVerbose: ::libc::c_int) -> ();
    pub fn wxLog_Suspend(_obj: *mut ::libc::c_void) -> ();
    pub fn wxProcess_Open(cmd: *mut ::libc::c_void, flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxProcess_IsErrorAvailable(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxProcess_IsInputAvailable(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxProcess_IsInputOpened(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxKill(pid: ::libc::c_int, signal: ::libc::c_int) -> ::libc::c_int;
    pub fn wxStreamBase_Delete(obj: *mut ::libc::c_void) -> ();
    pub fn wxGetColourFromUser(parent: *mut ::libc::c_void,
                               colInit: *mut ::libc::c_void,
                               colour: *mut ::libc::c_void) -> ();
    pub fn wxGetFontFromUser(parent: *mut ::libc::c_void,
                             fontInit: *mut ::libc::c_void,
                             font: *mut ::libc::c_void) -> ();
    pub fn wxGetPasswordFromUser(message: *mut ::libc::c_char,
                                 caption: *mut ::libc::c_char,
                                 defaultText: *mut ::libc::c_char,
                                 parent: *mut ::libc::c_void,
                                 _buf: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn wxGetTextFromUser(message: *mut ::libc::c_char,
                             caption: *mut ::libc::c_char,
                             defaultText: *mut ::libc::c_char,
                             parent: *mut ::libc::c_void, x: ::libc::c_int,
                             y: ::libc::c_int, center: ::libc::c_int,
                             _buf: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn wxGetNumberFromUser(message: *mut ::libc::c_void,
                               prompt: *mut ::libc::c_void,
                               caption: *mut ::libc::c_void,
                               value: ::libc::c_long, min: ::libc::c_long,
                               max: ::libc::c_long,
                               parent: *mut ::libc::c_void, x: ::libc::c_int,
                               y: ::libc::c_int) -> ::libc::c_long;
    pub fn wxcBell() -> ();
    pub fn wxcBeginBusyCursor() -> ();
    pub fn wxcEndBusyCursor() -> ();
    pub fn wxcIsBusy() -> ();
    pub fn wxTextCtrl_EmulateKeyPress(_obj: *mut ::libc::c_void,
                                      keyevent: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextCtrl_GetDefaultStyle(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextCtrl_GetRange(_obj: *mut ::libc::c_void,
                               from: ::libc::c_long, to: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn wxTextCtrl_GetStringSelection(_obj: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextCtrl_IsMultiLine(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextCtrl_IsSingleLine(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextCtrl_SetDefaultStyle(_obj: *mut ::libc::c_void,
                                      style: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextCtrl_SetMaxLength(_obj: *mut ::libc::c_void,
                                   len: ::libc::c_long) -> ();
    pub fn wxTextCtrl_SetStyle(_obj: *mut ::libc::c_void,
                               start: ::libc::c_long, end: ::libc::c_long,
                               style: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextAttr_Create(colText: *mut ::libc::c_void,
                             colBack: *mut ::libc::c_void,
                             font: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxTextAttr_CreateDefault() -> *mut ::libc::c_void;
    pub fn wxTextAttr_Delete(_obj: *mut ::libc::c_void) -> ();
    pub fn wxTextAttr_GetBackgroundColour(_obj: *mut ::libc::c_void,
                                          colour: *mut ::libc::c_void) -> ();
    pub fn wxTextAttr_GetFont(_obj: *mut ::libc::c_void,
                              font: *mut ::libc::c_void) -> ();
    pub fn wxTextAttr_GetTextColour(_obj: *mut ::libc::c_void,
                                    colour: *mut ::libc::c_void) -> ();
    pub fn wxTextAttr_HasBackgroundColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextAttr_HasFont(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextAttr_HasTextColour(_obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn wxTextAttr_IsDefault(_obj: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxTextAttr_SetTextColour(_obj: *mut ::libc::c_void,
                                    colour: *mut ::libc::c_void) -> ();
    pub fn wxTextAttr_SetBackgroundColour(_obj: *mut ::libc::c_void,
                                          colour: *mut ::libc::c_void) -> ();
    pub fn wxTextAttr_SetFont(_obj: *mut ::libc::c_void,
                              font: *mut ::libc::c_void) -> ();
    pub fn wxConfigBase_Get() -> *mut ::libc::c_void;
    pub fn wxConfigBase_Set(_self: *mut ::libc::c_void) -> ();
    pub fn wxFileConfig_Create(inp: *mut ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn wxBitmap_CreateFromImage(image: *mut ::libc::c_void,
                                    depth: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxImage_CreateFromDataEx(width: ::libc::c_int,
                                    height: ::libc::c_int,
                                    data: *mut ::libc::c_void,
                                    isStaticData: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxImage_Delete(image: *mut ::libc::c_void) -> ();
    pub fn wxColour_CreateFromInt(rgb: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxColour_GetInt(colour: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn wxColour_CreateFromUnsignedInt(rgba: uint32_t)
     -> *mut ::libc::c_void;
    pub fn wxColour_GetUnsignedInt(colour: *mut ::libc::c_void) -> uint32_t;
    pub fn wxcSystemSettingsGetColour(systemColour: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn wxcSetPixelRGB(buffer: *mut uint8_t, width: ::libc::c_int,
                          x: ::libc::c_int, y: ::libc::c_int,
                          rgb: ::libc::c_int) -> ();
    pub fn wxcGetPixelRGB(buffer: *mut uint8_t, width: ::libc::c_int,
                          x: ::libc::c_int, y: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wxcSetPixelRowRGB(buffer: *mut uint8_t, width: ::libc::c_int,
                             x: ::libc::c_int, y: ::libc::c_int,
                             rgbStart: ::libc::c_int, rgbEnd: ::libc::c_int,
                             count: ::libc::c_int) -> ();
    pub fn wxcInitPixelsRGB(buffer: *mut uint8_t, width: ::libc::c_int,
                            height: ::libc::c_int, rgba: ::libc::c_int) -> ();
    pub fn wxcSetPixelRGBA(buffer: *mut uint8_t, width: ::libc::c_int,
                           x: ::libc::c_int, y: ::libc::c_int, rgba: uint32_t)
     -> ();
    pub fn wxcGetPixelRGBA(buffer: *mut uint8_t, width: ::libc::c_int,
                           x: ::libc::c_int, y: ::libc::c_int) -> uint32_t;
    pub fn wxcSetPixelRowRGBA(buffer: *mut uint8_t, width: ::libc::c_int,
                              x: ::libc::c_int, y: ::libc::c_int,
                              rgbaStart: ::libc::c_int, rgbEnd: ::libc::c_int,
                              count: uint32_t) -> ();
    pub fn wxcInitPixelsRGBA(buffer: *mut uint8_t, width: ::libc::c_int,
                             height: ::libc::c_int, rgba: uint32_t) -> ();
    pub fn wxcMalloc(size: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn wxcFree(p: *mut ::libc::c_void) -> ();
    pub fn wxcWakeUpIdle() -> ();
    pub fn wxGetApplicationDir() -> *mut ::libc::c_void;
    pub fn wxGetApplicationPath() -> *mut ::libc::c_void;
    pub fn ELJApp_InitializeC(closure: *mut ::libc::c_void,
                              _argc: ::libc::c_int,
                              _argv: *mut *mut ::libc::c_char) -> ();
    pub fn ELJApp_GetIdleInterval() -> ::libc::c_int;
    pub fn ELJApp_SetIdleInterval(interval: ::libc::c_int) -> ();
}
