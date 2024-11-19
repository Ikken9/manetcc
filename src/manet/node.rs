use std::collections::HashSet;

pub type NodeId = usize;

pub struct Node {
    pub id: NodeId,                 
    pub transmission_rate: f64,      
    pub transmission_rate_limit: f64,     
    pub power_usage: f64,
    pub power_limit: f64, 
    pub neighbors: HashSet<NodeId>,
}

impl Node {
    pub fn new(id: NodeId, transmission_rate_limit: f64, power_limit: f64) -> Self {
        Self {
            id,
            transmission_rate: 0.0,
            transmission_rate_limit,
            power_usage: 0.0,
            power_limit,
            neighbors: HashSet::new(),
        }
    }

    pub fn calculate_local(&mut self) -> f64 {
        1.0
    }
    
    fn calculate_lagrangian(&mut self, f: f64, data_flow: f64, link_capacity: f64, zeta: f64, nu: f64) -> f64 {
        f + zeta * (data_flow - link_capacity) + nu * (self.power_usage - self.power_limit)
    }

    
    // R Partial derivate with respect to the Lagrangian
    fn calculate_lgr_r_partial(&mut self, r: f64, l: f64, alpha: f64, mu: f64, zeta: f64) {
        
        
        ((alpha * r / l) + mu * r) + zeta * limiting_factor
    }

    // L Partial derivate with respect to the Lagrangian
    fn calculate_lgr_l_partial(&mut self, l: f64, r: f64, alpha: f64, mu: f64, zeta: f64) -> f64 {
        let limiting_factor = if r < l { 0.0 } else { 1.0 };

        -(alpha * f64::powi(r, 2) / (2.0 * l) + mu * l) + zeta * limiting_factor
    }

    fn calculate_gradient(&mut self, r: f64, l: f64, alpha: f64, mu: f64) -> Vec<f64> {
        vec![self.calculate_f_r_partial(r, l, alpha, mu), self.calculate_f_l_partial(r, l ,alpha, mu)]
    }

    fn calculate_gradient_descent(&mut self, epsilon: f64, alpha: f64, mu: f64, eta: f64, k: u16) {
        let mut r = self.transmission_rate;
        let mut l = self.transmission_rate_limit;


        let gradient = self.calculate_gradient(r, l, alpha, mu);
        let partial_r = gradient[0];
        let partial_l = gradient[1];

        let magnitude = (f64::powi(partial_r, 2) + f64::powi(partial_l, 2)).sqrt(); // norm of the gradient

        let i = 0;
        while i < k {
            if magnitude > epsilon {
                break;
            }

            r = r - (eta * partial_r);
            l = l - (eta * partial_l);
        }
    }
    
    fn calculate_gradient_ascent(&mut self, epsilon: f64, ) {
        
    }
    
    fn make_projection() {
        
    }
    
    
}

