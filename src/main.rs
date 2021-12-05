use std::{fmt, fs::File};
use std::time::Duration;
use std::io::Write;
use libusb::{Context, Device, DeviceHandle};

fn main() -> Result<(), USBError> {
    // Get libusb context
    let context = Context::new()?;
    
    // Get list of devices
    let mut usb_list = USBList { list: vec![] };
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        let device_handle = context.open_device_with_vid_pid(device_desc.vendor_id(), device_desc.product_id()).unwrap();

        // For each USB device, get the information
        let usb_detail = get_device_information(device, &device_handle)?;
        usb_list.list.push(usb_detail);
    }
    println!("USB list: {}", usb_list);
    write_to_file(usb_list)?;
    Ok(())
    
}

// Function to print device information.
fn get_device_information(device: Device, device_handle: &DeviceHandle) -> Result<USBDetail, USBError> {
    let device_descriptor = device.device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = device_handle.read_languages(timeout)?;
    let language = languages[0];

    // Get device manufacturer name
    let manufacturer = device_handle.read_manufacturer_string(language, &device_descriptor, timeout)?;
    // Get device USB product name
    let product = device_handle.read_product_string(language, &device_descriptor, timeout)?;
    //Get product serial number
    let product_serial_number = match device_handle.read_serial_number_string(language, &device_descriptor, timeout) {
        Ok(s) => s,
        Err(_) => "Not available".into(),
    };
    // Populate the USBDetails struct
    Ok(USBDetail {
        manufacturer,
        product,
        serial_number: product_serial_number,
        bus_number: device.bus_number(),
        device_address: device.address(),
        vendor_id: device_descriptor.vendor_id(),
        product_id: device_descriptor.product_id(),
        maj_device_version:
        device_descriptor.device_version().0,
        min_device_version:
        device_descriptor.device_version().1,
    })
}

#[derive(Debug)]
struct USBError {
    err: String,
}

struct USBList {
    list: Vec<USBDetail>,
}

#[derive(Debug)]
struct USBDetail {
    manufacturer: String,
    product: String,
    serial_number: String,
    bus_number: u8,
    device_address: u8,
    vendor_id: u16,
    product_id: u16,
    maj_device_version: u8,
    min_device_version: u8, 
}

impl fmt::Display for USBList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for usb in &self.list {
            writeln!(f, "\nUSB Device detail")?;
            writeln!(f, "Manufacturer: {}", usb.manufacturer)?;
            writeln!(f, "Product: {}", usb.product)?;
            writeln!(f, "Serial number: {}", usb.serial_number)?;
            writeln!(f, "Bus number: {}", usb.bus_number)?;
            writeln!(f, "Device address: {}", usb.device_address)?;
            writeln!(f, "Vendor id: {}", usb.vendor_id)?;
            writeln!(f, "Product ID: {}", usb.product_id)?;
            writeln!(f, "Major device version: {}", usb.maj_device_version)?;
            writeln!(f, "Minor device version: {}", usb.min_device_version)?;
        })
    }
}

impl From<libusb::Error> for USBError {
    fn from(e: libusb::Error) -> Self {
        USBError {
            err: format!("Error in accessing USB device: {}", e),
        }
    }
}

impl From<std::io::Error> for USBError {
    fn from(e: std::io::Error) -> Self {
        USBError { err: e.to_string() }
    } 
}

// Function to write details to output file
fn write_to_file(usb: USBList) -> Result<(), USBError> {
    let mut file_handle = File::create("usb_details.txt")?;
    write!(file_handle, "{}\n", usb)?;
    Ok(())
}