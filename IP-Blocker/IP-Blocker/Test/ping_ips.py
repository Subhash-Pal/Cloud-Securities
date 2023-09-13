import json
import ping3

def ping_ip(ip_address):
    response_time = ping3.ping(ip_address)
    if response_time is not None:
        print(f"IP: {ip_address} - Response Time: {response_time} ms")
    else:
        print(f"IP: {ip_address} - Unreachable")

def main():
    json_file = "./ip_addresses.json"
    
    try:
        with open(json_file, "r") as file:
            data = json.load(file)
            ip_addresses = data["ip_addresses"]
    except FileNotFoundError:
        print(f"Error: The file '{json_file}' not found.")
        return
    except json.JSONDecodeError:
        print(f"Error: Unable to parse JSON from '{json_file}'.")
        return

    for ip in ip_addresses:
        ping_ip(ip)

if __name__ == "__main__":
    main()
