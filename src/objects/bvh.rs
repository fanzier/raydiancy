use std::f64;
use basic::*;
use objects::surface::*;

/// Represents a bounding volume hierarchy.
pub struct BVH<ContainerType: SurfaceContainer> {
    unbounded_objects: Vec<usize>,
    container: ContainerType,
    root_node: BVHNode,
}

struct BVHNode {
    pub bounding_box: Aabb,
    pub node: Box<BVHTreeNode>,
}

enum BVHTreeNode {
    pub Leaf {
        objects: Vec<usize>,
    },
    pub Branch {
        left: BVHNode,
        right: BVHNode,
    }
}

impl<ContainerType> BVH<ContainerType> where ContainerType: SurfaceContainer {
    pub fn new(container: ContainerType) -> BVH<ContainerType> {
        let mut unbounded_objects = vec![];
        let mut aabbs = vec![];
        for i in (0..container.count()) {
            let aabb = container.elem_bounding_box(i);
            match aabb {
                None => unbounded_objects.push(i),
                Some(aabb) => aabbs.push((i,aabb)),
            }
        }
        let root_node = BVHNode::new(&container, aabbs);
        BVH {
            unbounded_objects: unbounded_objects,
            container: container,
            root_node: root_node,
        }
    }

    fn node_is_hit_by(&self, node: &BVHNode, ray: Ray, t_max: f64) -> bool {
        if !node.bounding_box.passes_through(ray, t_max) {
            return false
        }
        match *node.node {
            BVHTreeNode::Leaf { ref objects } => {
                objects.iter().any(|&i| self.container.elem_is_hit_by(i, ray, t_max))
            },
            BVHTreeNode::Branch { ref left, ref right} => {
                unimplemented!();
            },
        }
    }

    fn node_intersect(&self, node: &BVHNode, ray: Ray, t_max: f64) -> Option<Intersection> {
        if !node.bounding_box.passes_through(ray, t_max) {
            return None
        }
        match *node.node {
            BVHTreeNode::Leaf { ref objects } => {
                let mut nearest_inter = node.bounding_box.intersect(ray, t_max);
                let mut nearest_t = match &nearest_inter {
                    &Some(ref inter) => inter.t,
                    &None => f64::INFINITY
                };
                for &i in objects {
                    match self.container.elem_intersect(i, ray, t_max) {
                        None => continue,
                        Some(inter) => if inter.t < nearest_t {
                            nearest_t = inter.t;
                            nearest_inter = Some(inter);
                        }
                    }
                }
                return nearest_inter
            },
            BVHTreeNode::Branch { ref left, ref right} => {
                unimplemented!();
            },
        }
    }
}

impl<ContainerType> Surface for BVH<ContainerType> where ContainerType: SurfaceContainer {
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        self.node_is_hit_by(&self.root_node, ray, t_max)
    }

    fn intersect(&self, ray: Ray, t_max: f64) -> Option <Intersection> {
        self.node_intersect(&self.root_node, ray, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.unbounded_objects.is_empty() {
            Some(self.root_node.bounding_box)
        } else {
            None
        }
    }
}

impl BVHNode {
    pub fn new<ContainerType>(container: &ContainerType, aabbs: Vec<(usize, Aabb)>) -> BVHNode {
        let ref mut iter = aabbs.iter();
        let start = iter.next().unwrap().1;
        let aabb = iter.fold(start, |acc, &(_, ref aabb)| {
            acc.union(aabb)
        });
        BVHNode {
            bounding_box: aabb,
            node: Box::new(BVHTreeNode::Leaf {
                objects: aabbs.iter().map(|x| x.0).collect(),
            }),
        }
    }
}

pub trait SurfaceContainer {
    /// Returns information about the intersection of the object and the ray, if one exists.
    /// If the distance is greater that `t_max`, it returns `None`.
    fn elem_intersect(&self, idx: usize, ray: Ray, t_max: f64) -> Option<Intersection>;

    /// Checks whether the ray intersects the object, computes no additional information.
    /// If the distance is greater than `t_max`, it returns `false`.
    fn elem_is_hit_by(&self, idx: usize, ray: Ray, t_max: f64) -> bool;

    /// Returns a finite (!) axis-aligned bounding box if one exists.
    fn elem_bounding_box(&self, idx: usize) -> Option<Aabb>;

    /// Returns the number of objects in the container.
    fn count(&self) -> usize;
}
