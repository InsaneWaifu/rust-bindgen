class PrimaryBase {
public:
    virtual void BaseMethod();
    virtual int Overridden(int value);
};

class SecondaryBase {
public:
    virtual float OtherMethod(float value) const;
};

class DerivedClass : public PrimaryBase {
public:
    int Overridden(int value) override;
    virtual void DerivedMethod();
};

class MultipleDerivedClass : public PrimaryBase, public SecondaryBase {
public:
    void BaseMethod() override;
    float OtherMethod(float value) const override;
    virtual unsigned MultiMethod(unsigned value);
};
