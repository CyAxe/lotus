function main(url)
    resp = send_req(url)
    print(string.match(resp.body, "SQL syntax.*MySQL"))
    return "tt"
end
