PAYLOADS = read(string.format("%s/txt/xss.txt",SCRIPT_PATH))


local function send_report(url,parameter,payload)
    NewReport:setName("reflected cross site scripting")
    NewReport:setDescription("https://owasp.org/www-community/attacks/xss/")
    NewReport:setRisk("medium")
    NewReport:setUrl(url)
    NewReport:setParam(parameter)
    NewReport:setAttack(payload)
    NewReport:setEvidence(generate_css_selector(payload))
end

function main(url)
    local resp = http:send("GET",HttpMessage:getUrl())
    if resp.errors:GetErrorOrNil() then
        local log_msg = string.format("[RXSS] Connection Error: %s",new_url)
        log_error(log_msg)
        return
    end

    local body = resp.body:GetStrOrNil()
    local headers = resp.headers:GetHeadersOrNil()
    local content_type = headers["content-type"]
    if content_type ~= nil then
        if string.find(content_type,"html") then
            for payload in PAYLOADS:gmatch("[^\n]+") do
                new_querys = HttpMessage:setAllParams(payload)
                for param_name, pay_url in pairs(new_querys) do
                    -- Generate Css Selector pattern to find the xss payload in the page
                    local css_pattern = generate_css_selector(payload)
                    if string.len(css_pattern) > 0 then
                        -- Search in the response body with the Css Selector pattern of the payload
                        local resp = http:send("GET", pay_url)
                        local body = resp.body:GetStrOrNil()
                        local searcher = html_search(body,css_pattern)
                        if string.len(searcher) > 0 then
                            println(string.format("RXSS: %s | %s | %s ",resp.url:GetStrOrNil(),current_payload,css_pattern))
                            send_report(resp.url:GetStrOrNil(),param_name,payload)
                            Reports:addReport(NewReport)
                        end

                    end
                end
            end
        end
    end
end
