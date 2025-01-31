--
-- Copyright (c) 2016 rxi
--
-- This library is free software; you can redistribute it and/or modify it
-- under the terms of the MIT license. See LICENSE for details.
--
local log = { _version = "0.1.0" }

local function embolden(text, color)
	local colors = {
		red = "31",
		green = "32",
		yellow = "33",
		blue = "34",
		magenta = "35",
		cyan = "36",
	}
	local color_code = colors[color] or "31" -- default to red if color not specified
	return string.format("\27[1;%sm%s\27[0m", color_code, text)
end

log.usecolor = true
log.outfile = nil
log.level = "trace"
log.showdate = false

local modes = {
	{ name = "trace", color = "\27[34m" },
	{ name = "debug", color = "\27[36m" },
	{ name = "info", color = "\27[32m" },
	{ name = "warn", color = "\27[33m" },
	{ name = "error", color = "\27[31m" },
	{ name = "fatal", color = "\27[35m" },
}

local levels = {}
for i, v in ipairs(modes) do
	levels[v.name] = i
end

local round = function(x, increment)
	increment = increment or 1
	x = x / increment
	return (x > 0 and math.floor(x + 0.5) or math.ceil(x - 0.5)) * increment
end

local _tostring = tostring
local tostring = function(...)
	local t = {}
	for i = 1, select("#", ...) do
		local x = select(i, ...)
		if type(x) == "number" then
			x = round(x, 0.01)
		end
		t[#t + 1] = _tostring(x)
	end
	return table.concat(t, " ")
end

for i, x in ipairs(modes) do
	local nameupper = "\27[1m" .. x.name:upper() .. "\27[0m"
	log[x.name] = function(...)
		-- Return early if we're below the log level
		if i < levels[log.level] then
			return
		end

		local msg = tostring(...)
		local info = debug.getinfo(2, "Sl")
		if not info then
			error("Failed to get debug info")
			return
		end

		local lineinfo = info.short_src .. embolden(" @", "magenta") .. info.currentline
		local timestr = log.showdate and "\27[3m" .. os.date("%H:%M:%S") .. "\27[0m|" or ""

		-- Output to console
		local ok, err = pcall(function()
			print(
				string.format(
					"%s▰%-6s▰%s%s %s %s %s",
					log.usecolor and x.color or "",
					nameupper,
					timestr,
					log.usecolor and "\27[0m" or "",
					embolden("IN", "magenta"),
					lineinfo,
					msg
				)
			)
		end)

		if not ok then
			error("Failed to output to console: " .. tostring(err))
		end

		-- Output to log file
		if log.outfile then
			local fp, open_err = io.open(log.outfile, "a")
			if not fp then
				error("Failed to open log file: " .. tostring(open_err))
				return
			end

			local ok, write_err = pcall(function()
				local datestr = log.showdate and os.date() .. "| " or ""
				local str = string.format("|%-6s%s@%s %s\n", nameupper, datestr, lineinfo, msg)
				fp:write(str)
			end)

			fp:close()

			if not ok then
				error("Failed to write to log file: " .. tostring(write_err))
			end
		end
	end
end

return log
