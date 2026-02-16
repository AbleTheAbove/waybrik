machine_state = {}

function on_build()
 print("Mud mixer placed.")
 machine_state.tick_count = 0
 local ge = GameEvent("NewInventory")
 ge:fire()
end

-- When an object is broken this fires.
function on_break()
end

-- when the game is saved. Save these object attributes.
-- also triggers when broken.
function on_save()
 
end

-- when the loaded is loaded. Load these object attributes
function on_load()
end



function on_tick()
 print("Mud machine ticked.")
end

function on_ten_tick()
end