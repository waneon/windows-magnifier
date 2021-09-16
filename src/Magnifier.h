#include <functional>
#include <utility>

class Magnifier {
   private:
    float mag_level;
    int dead_zone;

    std::function<int(int)> transform_x;
    std::function<int(int)> transform_y;

   public:
    static const float DEFAULT_MAG_LEVEL;
    static const int DEFAULT_DEAD_ZONE = 200;

    bool magnified;

    Magnifier(float mag_level = 1.5, int dead_zone = 200);
    ~Magnifier();

    void magnify(void) const;
    void unmagnify(void) const;
};