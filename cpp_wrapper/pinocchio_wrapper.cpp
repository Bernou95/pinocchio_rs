#include <pinocchio/multibody/sample-models.hpp>
#include <pinocchio/algorithm/joint-configuration.hpp>
#include <pinocchio/algorithm/rnea.hpp>
#include <Eigen/Core>

extern "C" {

// Opaque handle for use in Rust
struct PinHandle {
    pinocchio::Model  model;
    pinocchio::Data   data;
    Eigen::VectorXd   q;
    Eigen::VectorXd   v;
    Eigen::VectorXd   a;

    PinHandle()
        : data(model) {}
};

/// Create and initialize a sample manipulator model
PinHandle* pin_create_manipulator()
{
    auto *h = new PinHandle();
    pinocchio::buildModels::manipulator(h->model);
    h->data = pinocchio::Data(h->model);
    h->q = pinocchio::neutral(h->model);
    h->v = Eigen::VectorXd::Zero(h->model.nv);
    h->a = Eigen::VectorXd::Zero(h->model.nv);
    return h;
}

/// Free
void pin_destroy(PinHandle* h)
{
    delete h;
}

/// Set q, v, a from raw pointers
void pin_set_state(PinHandle* h,
                   const double* q,
                   const double* v,
                   const double* a)
{
    for(int i=0;i<h->model.nq;i++) h->q[i]=q[i];
    for(int i=0;i<h->model.nv;i++) h->v[i]=v[i];
    for(int i=0;i<h->model.nv;i++) h->a[i]=a[i];
}

/// Compute tau = rnea(...) ;  tau_out must have size = model.nv
void pin_compute_rnea(PinHandle* h, double* tau_out)
{
    Eigen::VectorXd tau = pinocchio::rnea(h->model,h->data,h->q,h->v,h->a);
    for(int i=0;i<h->model.nv;i++) tau_out[i] = tau[i];
}

/// Get the dimension of generalized velocity (nv)
int pin_get_nv(PinHandle* h)
{
    return h->model.nv;
}

} // extern "C"

