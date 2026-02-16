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
 machine_state.tick_count = machine_state.tick_count + 1
 if machine_state.tick_count >= 10 then
  machine_state.tick_count = 0
  -- Do the tenth tick magic.
  on_ten_tick() 
 end
 
 -- print("Mud mixer ticked. On tick "..machine_state.tick_count)
end

function on_ten_tick()
 -- TODO process inventory item a and inventory item b into mud
 -- TODO set up trace functionality so the logs can help
 -- print("Ten Tick")
 local temp_inventory = get_inventory()
 local item_in_liquid_slot = temp_inventory.slot["liquid"]
end