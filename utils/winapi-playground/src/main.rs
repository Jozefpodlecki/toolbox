#![allow(unsafe_op_in_unsafe_fn)]

use std::{mem::{self, MaybeUninit}, ptr::{self, addr_of_mut, null_mut}, slice};

use anyhow::{bail, Result};
use ntapi::ntobapi::{NtQueryObject, ObjectNameInformation, ObjectTypeInformation, ObjectTypesInformation, OBJECT_INFORMATION_CLASS, OBJECT_NAME_INFORMATION, OBJECT_TYPES_INFORMATION, OBJECT_TYPE_INFORMATION, POBJECT_TYPES_INFORMATION};
use winapi::shared::{ntdef::UNICODE_STRING, ntstatus::STATUS_INFO_LENGTH_MISMATCH};

use crate::api::enumerate_handles;

mod api;

// const OBJECT_TYPES_INFORMATION_CLASS: OBJECT_INFORMATION_CLASS = unsafe { mem::transmute(3u32) };

unsafe fn list_object_types() -> Vec<String> {

    let mut size = 0;

    let status = NtQueryObject(
        ptr::null_mut(),
        ObjectTypesInformation,
        ptr::null_mut(),
        0,
        &mut size,
    );


    println!("{} {}", status, size);
    // let info: OBJECT_TYPES_INFORMATION = Default::default();
    let mut info_ptr: POBJECT_TYPES_INFORMATION = Default::default();
    //   println!("{:?}", (*info).NumberOfTypes);
    // let mut buffer = vec![0u8; size as usize];

    // let status = NtQueryObject(
    //     ptr::null_mut(),
    //     ObjectTypesInformation,
    //     info as *mut _,
    //     0,
    //     &mut size,
    // );

    //  println!("{:?}", (*info).NumberOfTypes);

    // status = NtQueryObject(ptr::null_mut(), ObjectTypesInformation, buffer, 28, &mut size);

    vec![]
}

unsafe fn get_object_name(handle: usize) -> Option<String> {
    let mut size: u32 = 0;

    // First call to get required buffer size
    let status = NtQueryObject(
        handle as *mut _,
        ObjectNameInformation,
        ptr::null_mut(),
        0,
        &mut size,
    );

    if status != STATUS_INFO_LENGTH_MISMATCH {
        return None;
    }

    let mut buffer = vec![0u8; size as usize];

    let status = NtQueryObject(
        handle as *mut _,
        ObjectNameInformation,
        buffer.as_mut_ptr() as *mut _,
        size,
        &mut size,
    );

    if status != 0 {
        return None;
    }

    let name_info = buffer.as_ptr() as *const OBJECT_NAME_INFORMATION;
    if (*name_info).Name.Buffer.is_null() {
        return None;
    }

    let len = (*name_info).Name.Length / 2; // bytes → u16 count
    let slice = slice::from_raw_parts((*name_info).Name.Buffer, len as usize);
    Some(String::from_utf16_lossy(slice))
}

fn main() -> Result<()> {

    let handles = unsafe { enumerate_handles()? };

    let handle = handles.first().unwrap();

    unsafe {
        println!("{:?}", get_object_name(handle.handle));

        println!("{:?}", handle);
        let mut size = 0;
        let mut name_info = MaybeUninit::<OBJECT_NAME_INFORMATION>::uninit();
        addr_of_mut!((*name_info.as_mut_ptr()).Name).write(UNICODE_STRING {
            Length: 0,
            MaximumLength: 0,
            Buffer: null_mut(),
        });
        let mut name_info = name_info.assume_init();
        NtQueryObject(
            handle.handle as *mut winapi::ctypes::c_void,
            ObjectNameInformation,
            &mut name_info as *mut OBJECT_NAME_INFORMATION as *mut _,
            mem::size_of::<OBJECT_NAME_INFORMATION>() as u32,
            &mut size);
        println!("{:?}", size);

        if name_info.Name.Buffer.is_null() {
             println!("null");
            return Ok(());
        } else {
            let test = String::from_utf16(std::slice::from_raw_parts(
                name_info.Name.Buffer,
                (name_info.Name.Length / 2) as usize,
            )).unwrap();
            println!("{}", test);
        }
        // println!("{:?}", name_info.Name.);
        // NtQueryObject(
        //     handle.handle as *mut winapi::ctypes::c_void,
        //     ObjectTypeInformation,
        //      as *mut _,
        //     mem::size_of::<OBJECT_TYPE_INFORMATION>() as u32,
        //     &mut size);
        // println!("{:?}", size);
    }

    Ok(())
}
