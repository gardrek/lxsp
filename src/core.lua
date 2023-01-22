
local global = dofile"src/gmt.lua"

local _Core = nil
do
    local core = {
        std = {},

        Object = {},
            Nil = {},
            Atom = {},
                Symbol = {},
                Integer = {},
            List = {},
    }

    -- Object --
    
    function core.Object.is_atom(self)
        return false
    end

    -- Nil --
    
    core._nil = setmetatable({}, core.Nil)

    function core.Nil.index(self, key)
        if key == "is_atom" then
            local v = core.Atom[key]
            if v then return v end
        end

        local v = core.List[key]
        if v then return v end

        local v = core.Nil[key]
        if v then return v end
    end

    -- Atom --

    function core.Atom.is_atom(self)
        return true
    end

    -- List --
    
    function core.List.car(self)
        return self.car
    end

    function core.List.cdr(self)
        return self.cdr
    end

    -- std --

    function core.std.cons(a, b)
        return setmetatable({ car = a, cdr = b, }, core.List)
    end

    -- helper functions --

    function core.new_atom(ty, string)
        return setmetatable({ ty = ty, data = string, }, core.Atom)
    end

    function core.new_symbol(string)
        return core.new_atom(core.Symbol, string)
    end

    function core.new_integer(i)
        return core.new_atom(core.Integer, i)
    end

    _Core = core
end

local print_recursively
do
    local function indent(depth)
        return string.rep("    ", depth)
    end
    
    local function indent_helper(val, f, indent)
        local whitespace = string.rep("    ", indent)
        print(whitespace .. val)
    end

    local function tag(val)
        return "[Lua " .. type(val) .. ": " .. tostring(val) .. "]"
    end

    local function print_tagged_value(val, f, indent)
        indent_helper(tag(val), f, indent)
    end

    local function inspect(t, depth)
        local f = inspect
        depth = depth or 0

        if type(t) == "table" then
            string = "\n" .. indent(depth) .. "{\n"
            
            depth = depth + 1

            for key, value in pairs(t) do
                string = string .. indent(depth) .. "[" .. inspect(key, depth) .. "]: " .. inspect(value, depth) .. ", \n"
            end

            depth = depth - 1
            
            return string .. indent(depth) .. "}\n"
        --[[
        elseif type(t) == "function" then
            indent_helper("[Lua function]", f, indent)
        elseif type(t) == "CFunction" then
            indent_helper("[Lua-C FFI function]", f, indent)
        elseif type(t) == "userdata" then
            indent_helper("[userdata]", f, indent)
        elseif type(t) == "nil" then
            indent_helper("[Lua nil]", f, indent)
        elseif type(t) == "boolean" then
            print_tagged_value(tostring(t), f, indent)
        elseif type(t) == "string" then
            print_tagged_value("[=[" .. tostring(t) .. "]=]", f, indent)
        elseif type(t) == "number" then
            print_tagged_value(tostring(t), f, indent)
        else
            error("unhandled primitive Lua type " .. type(t))
        ]]
        else
            return tostring(t)
        end
    end
    
    print_recursively = inspect
end

-- inspect(_Core)

local x = { hello = 0, my = "name", is = function(inigo) return montoya end, { type"you" }, nil, my = false, [[prepare to die]] }

--[[
print(inspect(x))

print(x.my)

local test_mut = "string"

print(test_mut)

local function mutator()
    test_mut = "not a string?"
end

mutator()

print(test_mut) 
]]--

local dbg = require("src.dbg")

--[[

print(dbg.to_debug_string('say it to my "face"'))
print(dbg.to_debug_string('say it to my \"face\"'))
print(dbg.to_debug_string("say it to my \"face\""))

print(dbg.to_debug_string("say it to my 'face'"))
print(dbg.to_debug_string("say it to my \'face\'"))
print(dbg.to_debug_string('say it to my \'face\''))

print(dbg.to_debug_string(function (x) return x end))

print(dbg.to_debug_string({}))

local f = function(...)
    local args = table.pack(...)
    local printResult = "pr:"
    for i = 1, args.n do
        local v = args[i]
        printResult = printResult .. dbg.to_debug_string(v) .. ", "
    end
    return printResult
end

print(f(1, "shello", false, nil, function(x) return x end))

print(f(nil, nil, 1, "shello", false, nil, function(x) return x end))

print(f(1, "shello", false, nil, function(x) return x end, nil, nil))

--]]
