// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[derive(Clone)]
pub struct Event {
    pub event_type: _rt::String,
    pub movie_name: _rt::String,
    pub device_type: _rt::String,
    pub timestamp: _rt::String,
}
impl ::core::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Event")
            .field("event-type", &self.event_type)
            .field("movie-name", &self.movie_name)
            .field("device-type", &self.device_type)
            .field("timestamp", &self.timestamp)
            .finish()
    }
}
#[derive(Clone, Copy)]
pub enum EventType {
    Buffer,
    Play,
    Pause,
}
impl ::core::fmt::Debug for EventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            EventType::Buffer => f.debug_tuple("EventType::Buffer").finish(),
            EventType::Play => f.debug_tuple("EventType::Play").finish(),
            EventType::Pause => f.debug_tuple("EventType::Pause").finish(),
        }
    }
}
#[derive(Clone)]
pub struct EventDetails {
    pub event_type: EventType,
    pub movie_name: _rt::String,
    pub device_type: _rt::String,
    pub timestamp: _rt::String,
}
impl ::core::fmt::Debug for EventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EventDetails")
            .field("event-type", &self.event_type)
            .field("movie-name", &self.movie_name)
            .field("device-type", &self.device_type)
            .field("timestamp", &self.timestamp)
            .finish()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_latest_event_timestamp_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: i64,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::get_latest_event_timestamp(_rt::string_lift(bytes0), arg2 as u64);
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec3 = (result1.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr2.add(4).cast::<usize>() = len3;
    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_latest_event_timestamp<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_player_state_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::get_player_state(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec3 = (result1.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr2.add(4).cast::<usize>() = len3;
    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_player_state<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_latest_event_details_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::get_latest_event_details(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let Event {
        event_type: event_type3,
        movie_name: movie_name3,
        device_type: device_type3,
        timestamp: timestamp3,
    } = result1;
    let vec4 = (event_type3.into_bytes()).into_boxed_slice();
    let ptr4 = vec4.as_ptr().cast::<u8>();
    let len4 = vec4.len();
    ::core::mem::forget(vec4);
    *ptr2.add(4).cast::<usize>() = len4;
    *ptr2.add(0).cast::<*mut u8>() = ptr4.cast_mut();
    let vec5 = (movie_name3.into_bytes()).into_boxed_slice();
    let ptr5 = vec5.as_ptr().cast::<u8>();
    let len5 = vec5.len();
    ::core::mem::forget(vec5);
    *ptr2.add(12).cast::<usize>() = len5;
    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
    let vec6 = (device_type3.into_bytes()).into_boxed_slice();
    let ptr6 = vec6.as_ptr().cast::<u8>();
    let len6 = vec6.len();
    ::core::mem::forget(vec6);
    *ptr2.add(20).cast::<usize>() = len6;
    *ptr2.add(16).cast::<*mut u8>() = ptr6.cast_mut();
    let vec7 = (timestamp3.into_bytes()).into_boxed_slice();
    let ptr7 = vec7.as_ptr().cast::<u8>();
    let len7 = vec7.len();
    ::core::mem::forget(vec7);
    *ptr2.add(28).cast::<usize>() = len7;
    *ptr2.add(24).cast::<*mut u8>() = ptr7.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_latest_event_details<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
    let l2 = *arg0.add(8).cast::<*mut u8>();
    let l3 = *arg0.add(12).cast::<usize>();
    _rt::cabi_dealloc(l2, l3, 1);
    let l4 = *arg0.add(16).cast::<*mut u8>();
    let l5 = *arg0.add(20).cast::<usize>();
    _rt::cabi_dealloc(l4, l5, 1);
    let l6 = *arg0.add(24).cast::<*mut u8>();
    let l7 = *arg0.add(28).cast::<usize>();
    _rt::cabi_dealloc(l6, l7, 1);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_total_play_time_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::get_total_play_time(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            *ptr2.add(8).cast::<i64>() = _rt::as_i64(e);
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            let vec3 = (e.into_bytes()).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *ptr2.add(12).cast::<usize>() = len3;
            *ptr2.add(8).cast::<*mut u8>() = ptr3.cast_mut();
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_total_play_time<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => (),
        _ => {
            let l1 = *arg0.add(8).cast::<*mut u8>();
            let l2 = *arg0.add(12).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_total_play_time_of_movie_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: *mut u8,
    arg3: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let result2 =
        T::get_total_play_time_of_movie(_rt::string_lift(bytes0), _rt::string_lift(bytes1));
    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result2 {
        Ok(e) => {
            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
            match e {
                Some(e) => {
                    *ptr3.add(8).cast::<u8>() = (1i32) as u8;
                    *ptr3.add(16).cast::<i64>() = _rt::as_i64(e);
                }
                None => {
                    *ptr3.add(8).cast::<u8>() = (0i32) as u8;
                }
            };
        }
        Err(e) => {
            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
            let vec4 = (e.into_bytes()).into_boxed_slice();
            let ptr4 = vec4.as_ptr().cast::<u8>();
            let len4 = vec4.len();
            ::core::mem::forget(vec4);
            *ptr3.add(12).cast::<usize>() = len4;
            *ptr3.add(8).cast::<*mut u8>() = ptr4.cast_mut();
        }
    };
    ptr3
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_total_play_time_of_movie<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => (),
        _ => {
            let l1 = *arg0.add(8).cast::<*mut u8>();
            let l2 = *arg0.add(12).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_add_event_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: *mut u8,
    arg3: usize,
    arg4: *mut u8,
    arg5: usize,
    arg6: *mut u8,
    arg7: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let len2 = arg5;
    let bytes2 = _rt::Vec::from_raw_parts(arg4.cast(), len2, len2);
    let len3 = arg7;
    let bytes3 = _rt::Vec::from_raw_parts(arg6.cast(), len3, len3);
    let result4 = T::add_event(Event {
        event_type: _rt::string_lift(bytes0),
        movie_name: _rt::string_lift(bytes1),
        device_type: _rt::string_lift(bytes2),
        timestamp: _rt::string_lift(bytes3),
    });
    let ptr5 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result4 {
        Ok(e) => {
            *ptr5.add(0).cast::<u8>() = (0i32) as u8;
            let vec6 = (e.into_bytes()).into_boxed_slice();
            let ptr6 = vec6.as_ptr().cast::<u8>();
            let len6 = vec6.len();
            ::core::mem::forget(vec6);
            *ptr5.add(8).cast::<usize>() = len6;
            *ptr5.add(4).cast::<*mut u8>() = ptr6.cast_mut();
        }
        Err(e) => {
            *ptr5.add(0).cast::<u8>() = (1i32) as u8;
            let vec7 = (e.into_bytes()).into_boxed_slice();
            let ptr7 = vec7.as_ptr().cast::<u8>();
            let len7 = vec7.len();
            ::core::mem::forget(vec7);
            *ptr5.add(8).cast::<usize>() = len7;
            *ptr5.add(4).cast::<*mut u8>() = ptr7.cast_mut();
        }
    };
    ptr5
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_add_event<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
        }
        _ => {
            let l3 = *arg0.add(4).cast::<*mut u8>();
            let l4 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l3, l4, 1);
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_latest_time_of_cabi<T: Guest>(arg0: i32) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let v0 = match arg0 {
        0 => EventType::Buffer,
        1 => EventType::Play,
        n => {
            debug_assert_eq!(n, 2, "invalid enum discriminant");
            EventType::Pause
        }
    };
    let result1 = T::get_latest_time_of(v0);
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let EventDetails {
        event_type: event_type3,
        movie_name: movie_name3,
        device_type: device_type3,
        timestamp: timestamp3,
    } = result1;
    match event_type3 {
        EventType::Buffer => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
        }
        EventType::Play => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
        }
        EventType::Pause => {
            *ptr2.add(0).cast::<u8>() = (2i32) as u8;
        }
    }
    let vec4 = (movie_name3.into_bytes()).into_boxed_slice();
    let ptr4 = vec4.as_ptr().cast::<u8>();
    let len4 = vec4.len();
    ::core::mem::forget(vec4);
    *ptr2.add(8).cast::<usize>() = len4;
    *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
    let vec5 = (device_type3.into_bytes()).into_boxed_slice();
    let ptr5 = vec5.as_ptr().cast::<u8>();
    let len5 = vec5.len();
    ::core::mem::forget(vec5);
    *ptr2.add(16).cast::<usize>() = len5;
    *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
    let vec6 = (timestamp3.into_bytes()).into_boxed_slice();
    let ptr6 = vec6.as_ptr().cast::<u8>();
    let len6 = vec6.len();
    ::core::mem::forget(vec6);
    *ptr2.add(24).cast::<usize>() = len6;
    *ptr2.add(20).cast::<*mut u8>() = ptr6.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_latest_time_of<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(4).cast::<*mut u8>();
    let l1 = *arg0.add(8).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
    let l2 = *arg0.add(12).cast::<*mut u8>();
    let l3 = *arg0.add(16).cast::<usize>();
    _rt::cabi_dealloc(l2, l3, 1);
    let l4 = *arg0.add(20).cast::<*mut u8>();
    let l5 = *arg0.add(24).cast::<usize>();
    _rt::cabi_dealloc(l4, l5, 1);
}
pub trait Guest {
    fn get_latest_event_timestamp(event_type: _rt::String, user_id: u64) -> _rt::String;
    fn get_player_state(device_type: _rt::String) -> _rt::String;
    fn get_latest_event_details(device_type: _rt::String) -> Event;
    fn get_total_play_time(device_type: _rt::String) -> Result<u64, _rt::String>;
    fn get_total_play_time_of_movie(
        device_type: _rt::String,
        movie_name: _rt::String,
    ) -> Result<Option<u64>, _rt::String>;
    fn add_event(event_info: Event) -> Result<_rt::String, _rt::String>;
    fn get_latest_time_of(event_type: EventType) -> EventDetails;
}
#[doc(hidden)]

macro_rules! __export_world_example_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "get-latest-event-timestamp"]
    unsafe extern "C" fn export_get_latest_event_timestamp(arg0: *mut u8,arg1: usize,arg2: i64,) -> *mut u8 {
      $($path_to_types)*::_export_get_latest_event_timestamp_cabi::<$ty>(arg0, arg1, arg2)
    }
    #[export_name = "cabi_post_get-latest-event-timestamp"]
    unsafe extern "C" fn _post_return_get_latest_event_timestamp(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_latest_event_timestamp::<$ty>(arg0)
    }
    #[export_name = "get-player-state"]
    unsafe extern "C" fn export_get_player_state(arg0: *mut u8,arg1: usize,) -> *mut u8 {
      $($path_to_types)*::_export_get_player_state_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "cabi_post_get-player-state"]
    unsafe extern "C" fn _post_return_get_player_state(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_player_state::<$ty>(arg0)
    }
    #[export_name = "get-latest-event-details"]
    unsafe extern "C" fn export_get_latest_event_details(arg0: *mut u8,arg1: usize,) -> *mut u8 {
      $($path_to_types)*::_export_get_latest_event_details_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "cabi_post_get-latest-event-details"]
    unsafe extern "C" fn _post_return_get_latest_event_details(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_latest_event_details::<$ty>(arg0)
    }
    #[export_name = "get-total-play-time"]
    unsafe extern "C" fn export_get_total_play_time(arg0: *mut u8,arg1: usize,) -> *mut u8 {
      $($path_to_types)*::_export_get_total_play_time_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "cabi_post_get-total-play-time"]
    unsafe extern "C" fn _post_return_get_total_play_time(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_total_play_time::<$ty>(arg0)
    }
    #[export_name = "get-total-play-time-of-movie"]
    unsafe extern "C" fn export_get_total_play_time_of_movie(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,) -> *mut u8 {
      $($path_to_types)*::_export_get_total_play_time_of_movie_cabi::<$ty>(arg0, arg1, arg2, arg3)
    }
    #[export_name = "cabi_post_get-total-play-time-of-movie"]
    unsafe extern "C" fn _post_return_get_total_play_time_of_movie(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_total_play_time_of_movie::<$ty>(arg0)
    }
    #[export_name = "add-event"]
    unsafe extern "C" fn export_add_event(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,arg4: *mut u8,arg5: usize,arg6: *mut u8,arg7: usize,) -> *mut u8 {
      $($path_to_types)*::_export_add_event_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }
    #[export_name = "cabi_post_add-event"]
    unsafe extern "C" fn _post_return_add_event(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_add_event::<$ty>(arg0)
    }
    #[export_name = "get-latest-time-of"]
    unsafe extern "C" fn export_get_latest_time_of(arg0: i32,) -> *mut u8 {
      $($path_to_types)*::_export_get_latest_time_of_cabi::<$ty>(arg0)
    }
    #[export_name = "cabi_post_get-latest-time-of"]
    unsafe extern "C" fn _post_return_get_latest_time_of(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_latest_time_of::<$ty>(arg0)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 32]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 32]);
mod _rt {
    pub use alloc_crate::string::String;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }

    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }

    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }

    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }

    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }

    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_example_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_example_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 698] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xbc\x04\x01A\x02\x01\
A\x18\x01r\x04\x0aevent-types\x0amovie-names\x0bdevice-types\x09timestamps\x03\0\
\x05event\x03\0\0\x01q\x03\x06buffer\0\0\x04play\0\0\x05pause\0\0\x03\0\x0aevent\
-type\x03\0\x02\x01r\x04\x0aevent-type\x03\x0amovie-names\x0bdevice-types\x09tim\
estamps\x03\0\x0devent-details\x03\0\x04\x01@\x02\x0aevent-types\x07user-idw\0s\x04\
\0\x1aget-latest-event-timestamp\x01\x06\x01@\x01\x0bdevice-types\0s\x04\0\x10ge\
t-player-state\x01\x07\x01@\x01\x0bdevice-types\0\x01\x04\0\x18get-latest-event-\
details\x01\x08\x01j\x01w\x01s\x01@\x01\x0bdevice-types\0\x09\x04\0\x13get-total\
-play-time\x01\x0a\x01kw\x01j\x01\x0b\x01s\x01@\x02\x0bdevice-types\x0amovie-nam\
es\0\x0c\x04\0\x1cget-total-play-time-of-movie\x01\x0d\x01j\x01s\x01s\x01@\x01\x0a\
event-info\x01\0\x0e\x04\0\x09add-event\x01\x0f\x01@\x01\x0aevent-type\x03\0\x05\
\x04\0\x12get-latest-time-of\x01\x10\x04\x01.component:video-distribution-analyt\
ics/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\x0cproces\
sed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
