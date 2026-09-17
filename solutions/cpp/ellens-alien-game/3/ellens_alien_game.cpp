namespace targets {
namespace {
class Alien {
public:
    int x_coordinate;
    int y_coordinate;
    Alien(int x_value, int y_value) : x_coordinate(x_value), y_coordinate(y_value) {}

    [[nodiscard]] int get_health() const { return health; }

    bool hit() {
        if (health > 0) {
            --health;
        }
        return true;
    }

    [[nodiscard]] bool is_alive() const { return health > 0; }

    bool teleport(int x_new, int y_new) {
        x_coordinate = x_new;
        y_coordinate = y_new;
        return true;
    }

    [[nodiscard]] bool collision_detection(Alien other) const {
        return x_coordinate == other.x_coordinate && y_coordinate == other.y_coordinate;
    }

private:
    int health{3};
};
}  // namespace
}  // namespace targets
