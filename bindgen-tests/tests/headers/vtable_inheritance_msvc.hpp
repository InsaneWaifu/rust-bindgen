// bindgen-flags: -- --target=x86_64-pc-windows-msvc

class PrimaryBase {
public:
    virtual void RootMethod();
    virtual int Overridden(int value);
    virtual int Alloc(int size);
    virtual float Alloc(float size);
    virtual ~PrimaryBase();
};

class IntermediateClass : public PrimaryBase {
public:
    int Overridden(int value) override;
    int Alloc(int size) override;
    float Alloc(float size) override;
    virtual void IntermediateMethod();
    virtual ~IntermediateClass();
};

class LeafClass : public IntermediateClass {
public:
    virtual void LeafMethod();
    ~LeafClass() override;
};

class SecondaryBase {
public:
    virtual float SecondaryMethod(float value) const;
    virtual ~SecondaryBase();
};

class MultipleDerivedClass : public LeafClass, public SecondaryBase {
public:
    void RootMethod() override;
    float SecondaryMethod(float value) const override;
    virtual unsigned MultipleOnly(unsigned value);
    ~MultipleDerivedClass() override;
};
