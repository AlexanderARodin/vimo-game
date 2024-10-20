print("game script started..")

local target

local static_obstacles = {}
local dynamic_obstacles = {}

local shifts = {
	{1, 0},
	{1, 1},
	{0, 1},
	{0, 0},
}

function reset()
	target = {9,12}
	static_obstacles = {
		{0,0},
		{0,1},
		{1,0},
		{7,11},
		{7,12},
		{7,13},
	}
	dynamic_obstacles = {
		{9,9},
		{8,9},
		{9,13},
		{10,13},
		{10,12},
	}
end

function update(time)
	-- restarted
    if time == -1 then
         reset()
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
        target = target,
		obstacles = obstacles,
        GameOver,
    }
end
