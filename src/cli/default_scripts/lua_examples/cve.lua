SCAN_TYPE = 2 -- FUL URL

local function send_report(url) 
    -- Creating/Saving Script Report in the JSON Output
    Reports:add {
        name = "CVE-555-555",
        description = "https://example.com",
        url = url
    }
end


function main()
    local target_url = HttpMessage:Url()
    local response_body = "Hello World"
    local raw_response = "200 Ok\nServer: Nginx\n\n\nHello World"
    if str_startswith(raw_response,"200") == true and str_contains(response_body, "W") == true then
        println("THANKS FOR USING LOTUS, feel free to ask anything on our Github Issues page")
        send_report(target_url)
    end
end
