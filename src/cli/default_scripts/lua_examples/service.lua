SCAN_TYPE = 4 -- Custom Input Handler

local function send_report(tool_output)
  local input = INPUT_DATA -- USER INPUT from the custom input handler 
  Reports:add {
    ip = input,
    output = tool_output
  }
end

function main()
  local input = INPUT_DATA -- USER INPUT from the custom input handler 
  -- in this case we're gonna imagine that our input handler script output is IP address and we will send it to Another TOOL API To scan it
  local response = http:send{
    method = "POST",
    url = "http://1.2.3.4:6060/ips/scan", 
    headers = {
      KEY = "123"
    },
    body=string.format("ip=%s",input), -- ip=5.3.6.7
  }
  -- OUR API Stores the Scan ID in Location response header
  local scan_id = response.headers["location"]

  -- Printing The Scan ID to the User in CLI
  local log_msg = string.format("[INF] Scanning: %s under ID Number %s",input, scan_id)
  println(log_msg) -- Log with INFO Level
  log_info(log_msg)

  while true do
    -- Check if the scan task finished or not
    local check_done = http:send {url = string.format("http://1.2.3.4:6060/scans/%s", input)}
    local log_msg = string.format("[INF] Scanning Done: %s under ID Number %s",input, scan_id)
    if check_done.status == 200 then
      log_info(log_msg)
      println(log_msg)
      send_report(check_done.body)
      break
    else
      sleep(300) -- Sleeping 5 min
    end
  end
end

-- Json Output
--[[

$ cat out.json
[
    {
        ip = "5.3.6.7",
        output = "ssh port is open"
    }
]

--]]
