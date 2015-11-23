use basic::*;
use objects::surface::*;

/// The maximum depth for a bounding volume hierarchy.
const MAX_DEPTH: usize = 15;
/// The maximum number of objects in a BVH node.
/// Above this threshold, the node will be split if the depth limit allows it.
const COUNT_THRESHOLD: usize = 5;
/// If `true`, makes the BVH boxes visible (transparent red).
const DEBUG_BVH: bool = false;

/// Represents a bounding volume hierarchy.
pub struct Bvh<ContainerType: SurfaceContainer> {
    unbounded_objects: Vec<usize>, // TODO: Intersect those, too.
    container: ContainerType,
    root_node: BvhNode,
}

struct BvhNode {
    pub bounding_box: Aabb,
    pub node: Box<BvhTreeNode>,
}

enum BvhTreeNode {
    pub Leaf {
        objects: Vec<usize>,
    },
    pub Branch {
        left: BvhNode,
        right: BvhNode,
    }
}

impl<ContainerType> Bvh<ContainerType> where ContainerType: SurfaceContainer {
    /// Creates a BVH from a container.
    pub fn new(container: ContainerType) -> Bvh<ContainerType> {
        let mut unbounded_objects = vec![];
        let mut aabbs = vec![];
        for i in (0..container.count()) {
            let aabb = container.elem_bounding_box(i);
            match aabb {
                None => unbounded_objects.push(i),
                Some(aabb) => aabbs.push((i,aabb)),
            }
        }
        let root_node = BvhNode::new(&container, aabbs, MAX_DEPTH);
        Bvh {
            unbounded_objects: unbounded_objects,
            container: container,
            root_node: root_node,
        }
    }

    fn node_is_hit_by(&self, node: &BvhNode, ray: Ray, t_max: f64) -> bool {
        if !node.bounding_box.passes_through(ray, t_max) {
            return false
        }
        match *node.node {
            BvhTreeNode::Leaf { ref objects } => {
                objects.iter().any(|&i| self.container.elem_is_hit_by(i, ray, t_max))
            },
            BvhTreeNode::Branch { ref left, ref right} => {
                // TODO: Optimize: Check nearest node first and use t value for cutoff
                self.node_is_hit_by(left, ray, t_max) && self.node_is_hit_by(right, ray, t_max)
            },
        }
    }

    fn node_intersect<'a, 'b: 'a>(&'a self, node: &'b BvhNode, ray: Ray, t_max: f64) -> Option<DelayedIntersection> {
        if !node.bounding_box.passes_through(ray, t_max) {
            return None
        }
        let (t_max, no_intersection) = if DEBUG_BVH {
            let i = node.bounding_box.intersect(ray, t_max);
            match i {
                Some(i) => (i.t, Some(i)),
                None => (t_max, None),
            }
        } else {
            (t_max, None)
        };
        match *node.node {
            BvhTreeNode::Leaf { ref objects } => {
                let mut nearest_t = t_max;
                let mut nearest_inter = no_intersection;
                for &i in objects {
                    match self.container.elem_intersect(i, ray, nearest_t) {
                        None => continue,
                        Some(inter) => if inter.t < nearest_t {
                            nearest_t = inter.t;
                            nearest_inter = Some(inter);
                        }
                    }
                }
                return nearest_inter
            },
            BvhTreeNode::Branch { ref left, ref right} => {
                let (near, far) =
                    if left.bounding_box.distance(ray, t_max) < right.bounding_box.distance(ray, t_max) {
                        (left, right)
                    } else {
                        (right, left)
                    };
                match self.node_intersect(near, ray, t_max) {
                    None => self.node_intersect(far, ray, t_max).or(no_intersection),
                    Some(near_inter) => match self.node_intersect(far, ray, near_inter.t) {
                        None => Some(near_inter),
                        Some(far_inter) => Some(far_inter)
                    }
                }
            },
        }
    }
}

impl<ContainerType> Surface for Bvh<ContainerType> where ContainerType: SurfaceContainer {
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        self.node_is_hit_by(&self.root_node, ray, t_max)
    }

    fn intersect(&self, ray: Ray, t_max: f64) -> Option<DelayedIntersection> {
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

impl BvhNode {
    /// Creates a bounding volume hierarchy node,
    /// given a list of object indices with their bounding boxes.
    /// The node will recursively split until a depth of `max_depth`.
    pub fn new<ContainerType>(container: &ContainerType, aabbs: Vec<(usize, Aabb)>, max_depth: usize) -> BvhNode {
        let aabb = Aabb::union_all(&mut aabbs.iter().map(|&(_,b)| b));
        let tree_node = if aabbs.len() < COUNT_THRESHOLD || max_depth <= 0 {
            Box::new(BvhTreeNode::Leaf {
                objects: aabbs.iter().map(|x| x.0).collect(),
            })
        } else {
            let max = aabb.longest_side();
            let half = 0.5 * max.1 * Vec3::e(max.0);
            let left_half = Aabb::new(aabb.min(), aabb.max() - half);
            let right_half = Aabb::new(aabb.min() + half, aabb.max());
            let mut left_objects = vec![];
            let mut right_objects = vec![];
            for (i, bb) in aabbs {
                if !right_half.contains(&bb) {
                    left_objects.push((i, bb));
                }
                if !left_half.contains(&bb) {
                    right_objects.push((i, bb));
                }
            }
            Box::new(BvhTreeNode::Branch {
                left: BvhNode::new(container, left_objects, max_depth - 1),
                right: BvhNode::new(container, right_objects, max_depth - 1),
            })
        };
        BvhNode {
            bounding_box: aabb,
            node: tree_node,
        }
    }
}
