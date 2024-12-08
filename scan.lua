SCAN_TYPE = 3
BODY_MATCH = {
    'please input shell command', 'ZTE Corporation. All rights reserved'
}

local function send_report(resp)
    Reports:add{
        name = "CVE-2014-2321",
        description = "ZTE F460 and F660 cable modems allows remote attackers to obtain administrative access via sendcmd requests to web_shell_cmd.gch, as demonstrated by using 'set TelnetCfg' commands to enable a TELNET service with specified credentials.",
        risk = "high",
        url = resp.url,
        matches = {
            status_code = "200",
            body = {
                'please input shell command',
                'ZTE Corporation. All rights reserved'
            },
            full_response = show_response(resp)
        }
    }
end

local function scan_cve(target_path)
    local new_path = pathjoin(HttpMessage:path(), target_path)
    local new_url = HttpMessage:urljoin(new_path)
    local resp_status, resp = pcall(function()
        return http:send{url = new_url}
    end)
    if resp_status == true then
        local url = resp.url
        local body = resp.body
        local body_match = Matcher:match_body(body, BODY_MATCH) -- Matching body with List with and conditions
        if resp.status ~= 200 then return end
        if body_match then send_report(resp) end
    end
end

function main() 
    log_info("FFF")
    scan_cve("web_shell_cmd.gch")
end
