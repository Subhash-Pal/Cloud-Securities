use anyhow::Error;
use std::convert::TryInto;
use std::net::Ipv4Addr;
/*
Type                    :   Function
Name                    :   create_ipv4_addr
Parameter/Return Type   :   ip_addr: Ipv4Addr (IP Address)->
Description             :   It will take IP as parameter
                            and return an  32-bit unsigned integer type interger
*/
/// The function `create_ipv4_addr` takes an `Ipv4Addr` and converts it into a `u32` value.
///
/// Arguments:
///
/// * `ip_addr`: The `ip_addr` parameter is of type `Ipv4Addr`, which represents an IPv4 address.
///
/// Returns:
///
/// The function `create_ipv4_addr` returns a `Result<u32, Error>`.
pub fn create_ipv4_addr(ip_addr: Ipv4Addr) -> Result<u32, Error> {
    let octets = ip_addr.octets();
    let ip_addr: Ipv4Addr = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]).try_into()?;
    let ip_value: u32 = u32::from(ip_addr);
    Ok(ip_value)
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use super::create_ipv4_addr;

    #[test]
    fn test_create_ipv4_addr() {
        // Define test input
        let ip_addr = Ipv4Addr::new(192, 168, 0, 1);

        // Call the function and get the result
        let result = create_ipv4_addr(ip_addr);

        // Define the expected output
        let expected_output: u32 = 3232235521;

        // Assert that the result is equal to the expected output
        assert_eq!(result.unwrap(), expected_output);
    }
}


/// The function "create_ipv4_addr_param" converts an IPv4 address into a u32 value.
///
/// Arguments:
///
/// * `ip_addr`: The `ip_addr` parameter is of type `Ipv4Addr`, which represents an IPv4 address.
///
/// Returns:
///
/// a `Result<u32, Error>`.
pub fn create_ipv4_addr_param(ip_addr: Ipv4Addr) -> Result<u32, Error> {
    let ip_value: u32 = u32::from(ip_addr);
    Ok(ip_value)
}

#[test]
fn test_create_ipv4_addr_param() {
    // Test case 1: Valid IP address
    let ip_addr = Ipv4Addr::new(192, 168, 0, 1);
    let result = create_ipv4_addr_param(ip_addr);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3232235521);

    // Test case 2: Another valid IP address
    let ip_addr = Ipv4Addr::new(10, 0, 0, 1);
    let result = create_ipv4_addr_param(ip_addr);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 167772161);

    // Test case 3: Invalid IP address (out of range, not real IP)
    let ip_addr = Ipv4Addr::new(255, 0, 0, 1); 
    let result = create_ipv4_addr_param(ip_addr);
    assert!(result.is_ok());
    
}

#[cfg(test)]
mod tests1 {
    use std::net::Ipv4Addr;
    use super::create_ipv4_addr_param;

    #[test]
    fn test_create_ipv4_addr_param() {
        // Define test input
        let ip_addr = Ipv4Addr::new(192, 168, 0, 1);

        // Call the function and get the result
        let result = create_ipv4_addr_param(ip_addr);

        // Define the expected output
        let expected_output: u32 = 3232235521;

        // Assert that the result is equal to the expected output
        assert_eq!(result.unwrap(), expected_output);
    }
}





/*
fn main() {
    let ip_addr = create_ipv4_addr().unwrap();

    println!("Created Ipv4Addr: {}", ip_addr);

    // Use ip_addr variable here
    // ...
}*/
