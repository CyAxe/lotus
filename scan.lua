SCAN_TYPE = "PATHS" -- Updated to use string-based ScanTypes
BODY_MATCH = {
    "please input shell command", 
    "ZTE Corporation. All rights reserved"
}

-- Function to send the report for detected vulnerabilities
local function send_report(resp)
    reports:add{
        name = "CVE-2014-2321",
        description = "ZTE F460 and F660 cable modems allow remote attackers to obtain administrative access via sendcmd requests to web_shell_cmd.gch, as demonstrated by using 'set TelnetCfg' commands to enable a TELNET service with specified credentials.",
        risk = "high",
        url = resp.url,
        matches = {
            status_code = tostring(resp.status_code),
            body = BODY_MATCH,
            full_response = http:show_response(resp)
        }
    }
end

-- Function to perform the scan for the specified CVE
local function scan_cve(target_path)
    -- Attempt to send the HTTP request and capture the response
    local resp_status, resp = pcall(function()
        return http:send{
            url = "http://example.com/" .. target_path, -- Updated target URL construction
            method = "GET"
        }
    end)

    -- Check if the HTTP request was successful
    if resp_status and resp then
        -- Match status code and response body for potential vulnerabilities
        if resp.status_code == 200 then
            for _, pattern in ipairs(BODY_MATCH) do
                if string.match(resp.body, pattern) then
                    -- If a match is found, send the report
                    send_report(resp)
                    break
                end
            end
        end
    else
        -- Log the error if the HTTP request fails
    end
end

-- Main function to initiate the scan
function main()
    scan_cve("web_shell_cmd.gch")
end
