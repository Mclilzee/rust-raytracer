pub struct World {
    spheres: Vec<u8>,
}

impl World {
    pub fn init() -> Self {
        Self {
            spheres: Vec::default(),
        }
    }
    // pub fn drawSphere(self: *Self, translation: Vec3, radius: f64, material: *const Material) std.mem.Allocator.Error!void {
    //     try self.spheres.append(self.alloc, .{ .p = translation, .r = radius, .material = material });
    // }
    //
    // pub fn hit(self: Self, ray: *Ray) void {
    //     hitItems(self.spheres.items, ray);
    // }
    //
    // fn hitItems(array: anytype, ray: *Ray) void {
    //     for (array) |item| {
    //         item.hit(ray);
    //     }
    // }
}

trait Hittable {
    fn hit(&self);
}
