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

local function global(key, value)
    if value ~= nil then
        rawset(_G, key, value)
    else
        return rawget(_G, key)
    end
end

local mt = getmetatable(_G)

if mt == nil then
    mt = {}
    setmetatable(_G, mt)
end

mt.__newindex = function(self, key, value)
    if global("_ALLOW_GLOBAL_NEWINDEX") then
        rawset(self, key, value)
    else
        error('attempt to set uninitialized global ""' .. tostring(key) .. '": "' .. tostring(value) .. '"', 2)
    end
end

mt.__index = function(self, key)
    if global("_ALLOW_GLOBAL_INDEX") then
        return rawget(self, key)
    else
        error('No Globals is in effect ' .. tostring(key), 2)
    end
end

return global
