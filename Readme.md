# Rust Programming Test

## Gist

Given some initial landscape and some ours of rain compute the final state of the landscape


## Definitions

* There is a landscape divided in bunch of segments.
* Every segment has some height defined as a positive number.
* Once per hour a unit of water falls on each segment. 
* A unit of water increments in one the height of the segment.
* Water flows in units from the taller segments to the smaller ones
* There are infinite walls on the edges of the landscape

## Considerations

Although not in the problem definition in order to be able to solve the problem some things
must be taken into consideration:

* Water flows as it drops, that is after an hour of rain water flows to an steady position
* Water does not flow to equal height segments, this is water does not flow on plain surfaces.
  i.e: An unit of water in a segment of height 3 will not flow into a segment of height 3 

## Algorithm

Premise: This system reaches an steady state

Idea: Given some movement conditions, it's a matter of interacting the landscape over
and over moving the water. We will eventually find a steady-state where no water could
be moved from any of the segments.

Movement conditions:

 loop:
    If no units of water in segment
        break loop
    If right segment's height including water height is strictly less than my height including water:
        give the segment on the right one unit of water
        goto loop
    If left segment's height including water height is strictly less than my height including water:
        give the segment on the left one unit of water
        goto loop
    break loop
    
This can be expressed as:

  while ( myWaterUnits > 0 and have neighbourghs )
     neighbourg = retrieve neighbourg from list
     if neighbourg total height < my total height
         neighbourg waterUnits + 1
         waterUnits -1 
