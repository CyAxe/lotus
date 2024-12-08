SCAN_TYPE = "PATHS" -- Updated to use string-based ScanTypes
BODY_MATCH = {
    "please input shell command", 
    "ZTE Corporation. All rights reserved"
}
local http = lotus.http
local reports = lotus.reports
local matcher = lotus.matcher

local function send_report(resp)
    reports:add{
        name = "CVE-2014-2321",
        description = "ZTE F460 and F660 cable modems allow remote attackers to obtain administrative access via sendcmd requests to web_shell_cmd.gch, as demonstrated by using 'set TelnetCfg' commands to enable a TELNET service with specified credentials.",
        risk = "high",
        url = resp.url,
        matches = {
            status_code = "200",
            body = BODY_MATCH,
            full_response = http.show_response(resp)
        }
    }
end

local function scan_cve(target_path)
    local new_path = http.pathjoin(http:path(), target_path)
    local new_url = http:urljoin(new_path)

    local resp_status, resp = pcall(function()
        return http:send{url = new_url}
    end)

    if resp_status and resp.status == 200 then
        local body_match = matcher:match_body(resp.body, BODY_MATCH) -- Matching body with AND conditions
        if body_match then
            send_report(resp)
        end
    end
end

function main()
    lotus.log_debug("Starting CVE-2014-2321 Scan")
    scan_cve("web_shell_cmd.gch")
end
