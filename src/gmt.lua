--[[
    filename: gmt.lua
    description: this file sets up the global metatable which controls how global variables are accessed and created
]]

-- no globals allowed
-- stops globals from being created, but not from being used once created
-- so you can call this after all your globals are initialized and be fine

-- captures
local _G = _G
local error = error
local rawset = rawset
local rawget = rawget
local getmetatable = getmetatable
local setmetatable = setmetatable

local mt = getmetatable(_G)

if mt == nil then
    mt = {}
    setmetatable(_G, mt)
elseif type(mt) == "table" then
    local already_global = rawget(mt, "global")
    if type(already_global) == "function" then
        return already_global
    end
end

local function global(key, value)
    if value ~= nil then
        rawset(_G, key, value)
    else
        return rawget(_G, key)
    end
end

mt.global = global

mt.__newindex = function(self, key, value)
    if global"_ALLOW_GLOBAL_NEWINDEX" then
        rawset(self, key, value)
    else
        error('attempt to set uninitialized global "' .. tostring(key) .. '": "' .. tostring(value) .. '"', 2)
    end
end

mt.__index = function(self, key)
    print("GMT INDEX " .. key)
    if global"_ALLOW_GLOBAL_INDEX" then
        return rawget(self, key)
    else
        error("Attempt to access global '" .. tostring(key) .. "' while No Globals is in effect", 2)
    end
end

return global
