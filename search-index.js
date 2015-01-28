var searchIndex = {};
searchIndex['rust-conway'] = {"items":[[0,"","rust-conway",""],[0,"cell","",""],[4,"State","rust-conway::cell",""],[13,"Alive","","",0],[13,"Dead","","",0],[11,"eq","","",0],[11,"ne","","",0],[0,"world","rust-conway",""],[3,"HashWorld","rust-conway::world",""],[12,"cells","","",1],[8,"World","",""],[10,"get_cell","","",2],[10,"set_cell","","",2],[10,"kill_cell","","",2],[11,"num_adjacent","","",2],[10,"iter","","Returns a HashMap iterator of live cells.\nThis will stay in this format for the forseeable future.\nReturning a generic iterator adds too much overhead to the\niter() function",2],[11,"new","","",1],[11,"get_cell","","",1],[11,"set_cell","","",1],[11,"kill_cell","","",1],[11,"iter","","",1],[0,"engine","rust-conway",""],[3,"GrifLife","rust-conway::engine","The main implementation of Conway's game of life. This engine utilizes a list of updates to\ntrack interesting areas in the world. Only cells adjacent to cells that updated during the last\ngeneration are evaluated for a new state"],[8,"ConwayEngine","",""],[10,"next_generation","","Computes the next generation of cells, works on an internally held world::World object",3],[10,"world_ref","","Return an immutable reference to the internally held world object. This is useful for\ngetting access to the world so that It can be printed",3],[10,"world_ref_mut","","Return a mutable reference to the internally held world object.",3],[10,"set_cell","","Correct way to set the cell in the engine so we can know about the addition",3],[11,"next_generation","","",4],[11,"world_ref","","",4],[11,"world_ref_mut","","",4],[11,"set_cell","","",4],[11,"new","","Create a new instance of the engine, this should be used\non a world with an initial setup of cells.",4]],"paths":[[4,"State"],[3,"HashWorld"],[8,"World"],[8,"ConwayEngine"],[3,"GrifLife"]]};
initSearch(searchIndex);