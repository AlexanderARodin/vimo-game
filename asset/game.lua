print("game script started..")

local player
local target

local static_obstacles = {}
local dynamic_obstacles = {}

local shifts = {
	{1, 0},
	{1, 1},
	{0, 1},
	{0, 0},
}

function init()
	player = {2,2}
	target = {13,5}
	static_obstacles = {
		{0,0},
		{0,1},
		{1,0},
	}
	dynamic_obstacles = {
		{9,9},
		{8,9},
		{9,13},
	}
end

function update(time)
	-- restarted
    if time == -1 then
         init()
    end
	
	-- some dynamics
	local i = time % 4
	local shift = shifts[i+1]

	-- in-game objects location
	local obstacles = {}
	for i, item in ipairs(static_obstacles) do
		table.insert(obstacles, item)
	end
	for i, item in ipairs(dynamic_obstacles) do
		table.insert(obstacles, {item[1]+shift[1], item[2]+shift[2]})
	end

    return {
        player = player,
        target = target,
		obstacles = obstacles,
        GameOver,
    }
end

function action(ac)
    -- print("act:", ac)
    if ac == 1 then
        player = { player[1], player[2] - 1 }
    end
    if ac == 2 then
        player = { player[1], player[2] + 1 }
    end
    if ac == 3 then
        player = { player[1] - 1, player[2] }
    end
    if ac == 4 then
        player = { player[1] + 1, player[2] }
    end
end
