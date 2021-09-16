class MagInfo {
   private:
    float f;
    float mul_x;
    float mul_y;
    int sub_x;
    int sub_y;
    int max_x;
    int max_y;

   public:
    MagInfo();
    int TransformX(int) const;
    int TransformY(int) const;
    float GetMagFactor() const;
};