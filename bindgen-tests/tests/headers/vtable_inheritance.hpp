class BaseVtable {
public:
    virtual void BaseMethod();
    virtual int Overridden(int value);
};

class OtherBaseVtable {
public:
    virtual float OtherMethod(float value) const;
};

class DerivedVtable : public BaseVtable {
public:
    int Overridden(int value) override;
    virtual void DerivedMethod();
};

class MultiDerivedVtable : public BaseVtable, public OtherBaseVtable {
public:
    void BaseMethod() override;
    float OtherMethod(float value) const override;
    virtual unsigned MultiMethod(unsigned value);
};
